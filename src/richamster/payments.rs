use reqwest::StatusCode;
use secrecy::Secret;

use crate::api::token::Token;
use crate::api::{Api, PaymentsApi, RequestPath};
use crate::errors::RichamsterError;
use crate::models::payments::{
    ReplenishInfo, Withdraw, WithdrawDetailError, WithdrawError, WithdrawFieldError, WithdrawInfo,
    WithdrawResponse,
};
use crate::prepare_request;
use crate::richamster::common::{AuthState, HeaderCompose};
use crate::richamster::{common, replace_placeholder};

pub struct Payments {
    auth_state: AuthState,
}

impl Payments {
    pub fn with_jwt_token(token: String) -> Self {
        Self {
            auth_state: AuthState::JwtTokenAuth(common::JwtToken(Secret::new(token))),
        }
    }

    pub fn with_keys(api_key: String, secret_key: String) -> Self {
        Self {
            auth_state: AuthState::ApiSecretKeyAuth(
                common::ApiKey(Secret::new(api_key)),
                common::SecretKey(Secret::new(secret_key)),
            ),
        }
    }
}

impl Payments {
    pub async fn replenish_info(&self, token: Token) -> Result<ReplenishInfo, RichamsterError> {
        let mut url = Api::Payments(PaymentsApi::ReplenishInfo).full_url();
        let path_segments: Vec<&str> = url.path_segments().unwrap().collect();
        let new_path = replace_placeholder(
            path_segments,
            Into::<&str>::into(token).to_owned(),
            "{currency}",
        );
        url.set_path(new_path.as_str());

        let resp = prepare_request!(url, get)
            .compose(&self.auth_state)
            .send()
            .await?;
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
        let mut url = Api::Payments(PaymentsApi::WithdrawInfo).full_url();
        let path_segments: Vec<&str> = url.path_segments().unwrap().collect();
        let new_path = replace_placeholder(
            path_segments,
            Into::<&str>::into(token).to_owned(),
            "{currency}",
        );
        url.set_path(new_path.as_str());

        let resp = prepare_request!(url, get)
            .compose(&self.auth_state)
            .send()
            .await?;
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
        let mut url = Api::Payments(PaymentsApi::Withdraw).full_url();
        let path_segments: Vec<&str> = url.path_segments().unwrap().collect();
        let new_path = replace_placeholder(
            path_segments,
            Into::<&str>::into(token).to_owned(),
            "{currency}",
        );
        url.set_path(new_path.as_str());

        let withdraw = Withdraw {
            address,
            sum,
            fee: None,
            pin_code,
            minimum_confirmations: 5,
        };
        let payload = serde_json::to_string(&withdraw)?;
        let resp = prepare_request!(url, payload, post)
            .compose_with_payload(&self.auth_state, &payload)
            .send()
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
