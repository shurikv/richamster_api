use crate::api::token::Token;
use crate::api::{Api, PaymentsApi, RequestData, RequestPath};
use crate::errors::RichamsterError;
use crate::models::payments::{
    ReplenishInfo, Withdraw, WithdrawDetailError, WithdrawError, WithdrawFieldError, WithdrawInfo,
    WithdrawResponse,
};
use crate::richamster::common::{AuthState, HeaderCompose};
use crate::richamster::{common, replace_placeholder};
use reqwest::{Client, IntoUrl, Method, StatusCode};
use secrecy::Secret;
use std::convert::AsRef;

pub struct Payments {
    auth_state: AuthState,
    client: Client,
}

impl Payments {
    pub fn with_jwt_token(token: String) -> Self {
        Self {
            auth_state: AuthState::JwtTokenAuth(common::JwtToken(Secret::new(token))),
            client: Client::new(),
        }
    }

    pub fn with_keys(api_key: String, secret_key: String) -> Self {
        Self {
            auth_state: AuthState::ApiSecretKeyAuth(
                common::ApiKey(Secret::new(api_key)),
                common::SecretKey(Secret::new(secret_key)),
            ),
            client: Client::new(),
        }
    }
}

impl Payments {
    async fn send_request<U: IntoUrl>(
        &self,
        url: U,
        method: Method,
    ) -> Result<reqwest::Response, reqwest::Error> {
        self.client
            .request(method, url)
            .compose(&self.auth_state)
            .send()
            .await
    }

    async fn send_request_with_body<U: IntoUrl>(
        &self,
        url: U,
        method: Method,
        body: String,
    ) -> Result<reqwest::Response, reqwest::Error> {
        self.client
            .request(method, url)
            .body(body.clone())
            .header("Content-Type", "application/json")
            .compose_with_payload(&self.auth_state, body.as_str())
            .send()
            .await
    }

    pub async fn replenish_info(&self, token: Token) -> Result<ReplenishInfo, RichamsterError> {
        let RequestData(mut url, method) = Api::Payments(PaymentsApi::ReplenishInfo).request_data();
        url = url.join(token.as_ref())?;
        let resp = self.send_request(url, method).await?;
        match resp.status() {
            StatusCode::OK => {
                let string = resp.text().await?;
                let balance: ReplenishInfo = serde_json::from_str(&string)?;
                Ok(balance)
            }
            StatusCode::UNAUTHORIZED => Err(RichamsterError::UnauthorizedAccess),
            status => Err(RichamsterError::UnsupportedResponseCode(
                status,
                resp.text().await?,
            )),
        }
    }

    pub async fn withdraw_info(&self, token: Token) -> Result<WithdrawInfo, RichamsterError> {
        let RequestData(mut url, method) = Api::Payments(PaymentsApi::WithdrawInfo).request_data();
        url = url.join(token.as_ref())?;

        let resp = self.send_request(url, method).await?;
        match resp.status() {
            StatusCode::OK => {
                let string = resp.text().await?;
                let balance: WithdrawInfo = serde_json::from_str(&string)?;
                Ok(balance)
            }
            StatusCode::UNAUTHORIZED => Err(RichamsterError::UnauthorizedAccess),
            status => Err(RichamsterError::UnsupportedResponseCode(
                status,
                resp.text().await?,
            )),
        }
    }

    pub async fn withdraw(
        &self,
        address: String,
        sum: String,
        pin_code: String,
        token: Token,
    ) -> Result<WithdrawResponse, RichamsterError> {
        let RequestData(mut url, method) = Api::Payments(PaymentsApi::Withdraw).request_data();
        let path_segments: Vec<&str> = url.path_segments().unwrap().collect();
        let new_path = replace_placeholder(path_segments, token.as_ref().to_owned(), "{currency}");
        url.set_path(new_path.as_str());

        let withdraw = Withdraw {
            address,
            sum,
            fee: None,
            pin_code,
            minimum_confirmations: 5,
        };
        let resp = self
            .send_request_with_body(url, method, serde_json::to_string(&withdraw)?)
            .await?;
        match resp.status() {
            StatusCode::CREATED => {
                let withdraw: WithdrawResponse = serde_json::from_str(resp.text().await?.as_str())?;
                Ok(withdraw)
            }
            StatusCode::BAD_REQUEST => {
                let error: WithdrawFieldError = serde_json::from_str(resp.text().await?.as_str())?;
                Err(RichamsterError::WithdrawError(WithdrawError::Fields(error)))
            }
            StatusCode::UNAUTHORIZED => Err(RichamsterError::UnauthorizedAccess),
            StatusCode::FORBIDDEN => {
                let detail: WithdrawDetailError =
                    serde_json::from_str(resp.text().await?.as_str())?;
                Err(RichamsterError::WithdrawError(WithdrawError::Detail(
                    detail,
                )))
            }
            status => Err(RichamsterError::UnsupportedResponseCode(
                status,
                resp.text().await?,
            )),
        }
    }
}
