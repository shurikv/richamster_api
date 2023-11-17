use crate::api::token::Token;
use crate::api::{Api, RequestData, RequestPath, TransferApi};
use crate::errors::RichamsterError;
use crate::models::transfer::TransferQuery;
use crate::richamster::common::{ApiKey, AuthState, HeaderCompose, JwtToken, SecretKey};
use crate::send_request;
use secrecy::Secret;
use std::convert::AsRef;

pub struct Transfer {
    auth_state: AuthState,
}

impl Transfer {
    pub fn new(jwt: String, api_key: String, secret_key: String) -> Self {
        Self {
            auth_state: AuthState::JwtTokenWithApiSecretKeyAuth(
                JwtToken(Secret::new(jwt)),
                ApiKey(Secret::new(api_key)),
                SecretKey(Secret::new(secret_key)),
            ),
        }
    }
}

impl Transfer {
    pub async fn transfer(
        &self,
        amount: u64,
        token: Token,
        to: String,
        pin_code: String,
    ) -> Result<(), RichamsterError> {
        let RequestData(url, method) = Api::Transfer(TransferApi::Transfer).request_data();
        let transfer_query = TransferQuery {
            amount: amount.to_string(),
            currency: token.as_ref().to_owned(),
            to,
            pin_code,
        };
        let payload = serde_json::to_string(&transfer_query)?;
        send_request!(url, method, self.auth_state, payload);
        Ok(())
    }
}
