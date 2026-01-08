use crate::api::UserApi::{Balances, Detail, Orders, Transactions, Transfer};
use crate::api::{Api, RequestPath};
use crate::api::{RequestData, token};
use crate::errors::RichamsterError;
use crate::models::user::{
    TransactionsFilter, TransferQuery, UserBalance, UserDetail, UserOrderResponse,
    UserOrdersFilter, UserTransactionResponce,
};
use crate::richamster::common;
use crate::richamster::common::{ApiKey, AuthState, HeaderCompose, JwtToken, SecretKey};
use crate::{process_response, send_request};
use reqwest::StatusCode;
use secrecy::SecretBox;

#[derive(Default)]
pub struct User {
    auth_state: AuthState,
}

impl User {
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

    pub fn with_jwt_and_keys(jwt: String, api_key: String, secret_key: String) -> Self {
        Self {
            auth_state: AuthState::JwtTokenWithApiSecretKeyAuth(
                JwtToken(SecretBox::new(Box::new(jwt))),
                ApiKey(SecretBox::new(Box::new(api_key))),
                SecretKey(SecretBox::new(Box::new(secret_key))),
            ),
        }
    }
}

impl User {
    pub async fn balances(
        &self,
        currency: Option<token::Token>,
    ) -> Result<Vec<UserBalance>, RichamsterError> {
        let RequestData(mut url, method) = Api::User(Balances).request_data();
        if let Some(token) = currency {
            url.query_pairs_mut()
                .append_pair("currency", token.as_ref());
        }
        let resp = send_request!(url, method, self.auth_state);
        process_response!(resp, Vec<UserBalance>)
    }

    pub async fn detail_info(&self) -> Result<UserDetail, RichamsterError> {
        let RequestData(url, method) = Api::User(Detail).request_data();
        let resp = send_request!(url, method, self.auth_state);
        process_response!(resp, UserDetail)
    }

    pub async fn transactions_list(
        &self,
        parameters: TransactionsFilter,
    ) -> Result<UserTransactionResponce, RichamsterError> {
        let RequestData(mut url, method) = Api::User(Transactions).request_data();
        let url = parameters.compose_url(&mut url);
        let resp = send_request!(url, method, self.auth_state);
        process_response!(resp, UserTransactionResponce)
    }

    pub async fn orders(
        &self,
        parameters: UserOrdersFilter,
    ) -> Result<UserOrderResponse, RichamsterError> {
        let RequestData(mut url, method) = Api::User(Orders).request_data();
        let url = parameters.compose_url(&mut url);
        println!("Composed URL: {}", url);
        let resp = send_request!(url, method, self.auth_state);
        let string = resp.text().await?;
        Ok(serde_json::from_str(&string)?)
    }

    pub async fn transfer(&self, transfer_query: TransferQuery) -> Result<(), RichamsterError> {
        let RequestData(url, method) = Api::User(Transfer).request_data();
        let payload = serde_json::to_string(&transfer_query)?;
        send_request!(url, method, self.auth_state, payload);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_default_user() {
        let user: User = Default::default();
        match user.auth_state {
            AuthState::Unauthorized => assert!(true),
            _ => assert!(false),
        }
    }
}
