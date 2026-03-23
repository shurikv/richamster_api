use crate::api::UserApi::{Balances, Detail, Orders, Transactions, Transfer};
use crate::api::{Api, RequestPath};
use crate::api::{RequestData, token};
use crate::errors::RichamsterError;
use crate::models::user::{
    TransactionsFilter, TransferQuery, UserBalance, UserDetail, UserOrderResponse,
    UserOrdersFilter, UserTransactionResponse,
};
use crate::richamster::common::{
    ApiClient, AuthState, process_response, send_request_with_auth, send_request_with_body_and_auth,
};

#[derive(Default)]
pub struct User {
    auth_state: AuthState,
}

impl ApiClient for User {
    fn from_auth_state(auth_state: AuthState) -> Self {
        Self { auth_state }
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
        let resp = send_request_with_auth(url, method, &self.auth_state).await?;
        process_response(resp).await
    }

    pub async fn detail_info(&self) -> Result<UserDetail, RichamsterError> {
        let RequestData(url, method) = Api::User(Detail).request_data();
        let resp = send_request_with_auth(url, method, &self.auth_state).await?;
        process_response(resp).await
    }

    pub async fn transactions_list(
        &self,
        parameters: TransactionsFilter,
    ) -> Result<UserTransactionResponse, RichamsterError> {
        let RequestData(mut url, method) = Api::User(Transactions).request_data();
        let url = parameters.compose_url(&mut url);
        let resp = send_request_with_auth(url, method, &self.auth_state).await?;
        process_response(resp).await
    }

    pub async fn orders(
        &self,
        parameters: UserOrdersFilter,
    ) -> Result<UserOrderResponse, RichamsterError> {
        let RequestData(mut url, method) = Api::User(Orders).request_data();
        let url = parameters.compose_url(&mut url);
        let resp = send_request_with_auth(url, method, &self.auth_state).await?;
        let string = resp.text().await?;
        Ok(serde_json::from_str(&string)?)
    }

    pub async fn transfer(&self, transfer_query: TransferQuery) -> Result<(), RichamsterError> {
        let RequestData(url, method) = Api::User(Transfer).request_data();
        let payload = serde_json::to_string(&transfer_query)?;
        send_request_with_body_and_auth(url, method, payload, &self.auth_state).await?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_default_user() {
        let user: User = Default::default();
        assert!(matches!(user.auth_state, AuthState::Unauthorized));
    }
}
