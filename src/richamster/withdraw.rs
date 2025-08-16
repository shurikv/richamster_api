use crate::api::token::Token;
use crate::api::{Api, RequestData, RequestPath, WithdrawApi};
use crate::errors::RichamsterError;
use crate::models::withdraw::{WithdrawData, WithdrawDetailError, WithdrawError, WithdrawFieldError, WithdrawInfoResponse, WithdrawResponse};
use crate::richamster::common;
use crate::richamster::common::AuthState;
use crate::richamster::common::HeaderCompose;
use crate::send_request;
use reqwest::StatusCode;
use secrecy::SecretBox;

#[derive(Default)]
pub struct Withdraw {
    auth_state: AuthState,
}

impl Withdraw {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_jwt_token(token: String) -> Self {
        Self {
            auth_state: AuthState::JwtTokenAuth(common::JwtToken(SecretBox::new(Box::new(token)))),
        }
    }

    pub fn with_keys(api_key: String, secret_key: String) -> Self {
        Self {
            auth_state: AuthState::ApiSecretKeyAuth(
                common::ApiKey(SecretBox::new(Box::new(api_key))),
                common::SecretKey(SecretBox::new(Box::new(secret_key))),
            ),
        }
    }
}

impl Withdraw {
    pub async fn withdraw_info(
        &self,
        token: Token,
    ) -> Result<WithdrawInfoResponse, RichamsterError> {
        let RequestData(mut url, method) = Api::Withdraw(WithdrawApi::WithdrawInfo).request_data();
        url = url.join(token.as_ref())?;

        let resp = send_request!(url, method, self.auth_state);
        match resp.status() {
            StatusCode::OK => {
                let string = resp.text().await?;
                let info: WithdrawInfoResponse = serde_json::from_str(&string)?;
                Ok(info)
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
        token: Token,
        withdraw: WithdrawData,
    ) -> Result<WithdrawResponse, RichamsterError> {
        let RequestData(mut url, method) = Api::Withdraw(WithdrawApi::Withdraw).request_data();
        url = url.join(token.as_ref())?;

        let resp = send_request!(
            url,
            method,
            self.auth_state,
            serde_json::to_string(&withdraw)?
        );
        match resp.status() {
            StatusCode::OK => {
                let string = resp.text().await?;
                let withdraw: WithdrawResponse = serde_json::from_str(&string)?;
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
