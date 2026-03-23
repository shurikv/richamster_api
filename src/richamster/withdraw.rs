use crate::api::token::Token;
use crate::api::{Api, RequestData, RequestPath, WithdrawApi};
use crate::errors::RichamsterError;
use crate::models::withdraw::{
    WithdrawData, WithdrawDetailError, WithdrawError, WithdrawFieldError, WithdrawInfoResponse,
    WithdrawResponse,
};
use crate::richamster::common::{
    ApiClient, AuthState, send_request_with_auth, send_request_with_body_and_auth,
};
use reqwest::StatusCode;

#[derive(Default)]
pub struct Withdraw {
    auth_state: AuthState,
}

impl ApiClient for Withdraw {
    fn from_auth_state(auth_state: AuthState) -> Self {
        Self { auth_state }
    }
}

impl Withdraw {
    pub async fn withdraw_info(
        &self,
        token: Token,
    ) -> Result<WithdrawInfoResponse, RichamsterError> {
        let RequestData(mut url, method) = Api::Withdraw(WithdrawApi::WithdrawInfo).request_data();
        url = url.join(token.as_ref())?;

        let resp = send_request_with_auth(url, method, &self.auth_state).await?;
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

        let resp = send_request_with_body_and_auth(
            url,
            method,
            serde_json::to_string(&withdraw)?,
            &self.auth_state,
        )
        .await?;
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
