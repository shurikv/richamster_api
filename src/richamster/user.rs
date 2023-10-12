use crate::api::UserApi::Detail;
use crate::api::UserApi::Transactions;
use crate::api::UserApi::{Balances, Orders};
use crate::api::{token, RequestData};
use crate::api::{Api, RequestPath};
use crate::errors::RichamsterError;
use crate::models::user::{
    OrdersFilter, TransactionsFilter, UserBalance, UserDetail, UserOrder, UserTransaction,
};
use crate::richamster::common;
use crate::richamster::common::{AuthState, HeaderCompose};
use crate::{process_response, send_request};
use reqwest::StatusCode;
use secrecy::Secret;

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
    ) -> Result<Vec<UserTransaction>, RichamsterError> {
        let RequestData(mut url, method) = Api::User(Transactions).request_data();
        let url = parameters.compose_url(&mut url);
        let resp = send_request!(url, method, self.auth_state);
        process_response!(resp, Vec<UserTransaction>)
    }

    pub async fn orders(
        &self,
        parameters: OrdersFilter,
    ) -> Result<Vec<UserOrder>, RichamsterError> {
        let RequestData(mut url, method) = Api::User(Orders).request_data();
        let url = parameters.compose_url(&mut url);
        let resp = send_request!(url, method, self.auth_state);
        process_response!(resp, Vec<UserOrder>)
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
