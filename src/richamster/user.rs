use crate::api::token;
use crate::api::UserApi::Detail;
use crate::api::UserApi::Transactions;
use crate::api::UserApi::{Balances, Orders};
use crate::api::{Api, RequestPath};
use crate::errors::RichamsterError;
use crate::models::user::{
    OrdersFilter, TransactionsFilter, UserBalance, UserDetail, UserOrder, UserTransaction,
};
use crate::richamster::common;
use crate::richamster::common::{AuthState, HeaderCompose};
use crate::{prepare_request, process_response};
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
        let mut url = Api::User(Balances).full_url();
        if let Some(token) = currency {
            url = url
                .join(format!("?currency={}", token.as_ref()).as_str())
                .unwrap();
        }

        let resp = prepare_request!(url, get)
            .compose(&self.auth_state)
            .send()
            .await?;
        process_response!(resp, Vec<UserBalance>)
    }

    pub async fn detail_info(&self) -> Result<UserDetail, RichamsterError> {
        let url = Api::User(Detail).full_url();
        let resp = prepare_request!(url, get)
            .compose(&self.auth_state)
            .send()
            .await?;
        process_response!(resp, UserDetail)
    }

    pub async fn transactions_list(
        &self,
        parameters: TransactionsFilter,
    ) -> Result<Vec<UserTransaction>, RichamsterError> {
        let mut url = Api::User(Transactions).full_url();
        let params = parameters.compose_url();
        if !params.is_empty() {
            url = url.join(format!("?{}", params).as_str())?;
        }
        let resp = prepare_request!(url, get)
            .compose(&self.auth_state)
            .send()
            .await?;

        process_response!(resp, Vec<UserTransaction>)
    }

    pub async fn orders(
        &self,
        parameters: OrdersFilter,
    ) -> Result<Vec<UserOrder>, RichamsterError> {
        let mut url = Api::User(Orders).full_url();
        let params = parameters.compose_url();
        if !params.is_empty() {
            url = url.join(format!("?{}", params).as_str())?;
        }
        let resp = prepare_request!(url, get)
            .compose(&self.auth_state)
            .send()
            .await?;

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
