use crate::api::token::CurrencyPair;
use crate::api::ExchangeApi;
use crate::api::{token, RequestData};
use crate::api::{Api, RequestPath};
use crate::errors::RichamsterError;
use crate::models::exchange::{
    CurrencyInfo, CurrencyPairRestriction, FavouritePairResponse, Market, NewOrder, NewOrderError,
    OrderBookFilter, OrdersBook, OrdersFilter, OrdersHistory, Ticker,
};
use crate::richamster::common::{ApiKey, AuthState, HeaderCompose, JwtToken, SecretKey};
use crate::richamster::replace_placeholder;
use crate::{process_response, send_request};
use reqwest::{Client, IntoUrl, Method, StatusCode};
use secrecy::Secret;
use url::Url;

#[derive(Default)]
pub struct Exchange {
    auth_state: AuthState,
    client: Client,
}

impl Exchange {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_jwt_token(token: String) -> Self {
        Self {
            auth_state: AuthState::JwtTokenAuth(JwtToken(Secret::new(token))),
            ..Default::default()
        }
    }

    pub fn with_keys(api_key: String, secret_key: String) -> Self {
        Self {
            auth_state: AuthState::ApiSecretKeyAuth(
                ApiKey(Secret::new(api_key)),
                SecretKey(Secret::new(secret_key)),
            ),
            ..Default::default()
        }
    }
}

impl Exchange {
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

    pub async fn restrictions_list(&self) -> Result<Vec<CurrencyPairRestriction>, RichamsterError> {
        let RequestData(url, method) = Api::Exchange(ExchangeApi::Restrictions).request_data();
        Ok(self.send_request(url, method).await?.json().await?)
    }

    pub async fn ticker_list(
        &self,
        pair: Option<CurrencyPair>,
    ) -> Result<Vec<Ticker>, RichamsterError> {
        let RequestData(mut url, method) = Api::Exchange(ExchangeApi::TickerList).request_data();
        if let Some(pair) = pair {
            url.query_pairs_mut()
                .append_pair("pair", pair.to_string().as_str());
        }
        Ok(self.send_request(url, method).await?.json().await?)
    }

    pub async fn favourites_pair_toggle(
        &self,
        pair: CurrencyPair,
    ) -> Result<FavouritePairResponse, RichamsterError> {
        let market_list = self.markets_list().await?;
        let market = if let Some(m) = market_list
            .iter()
            .find(|m| m.abbreviation == pair.to_string())
        {
            m
        } else {
            return Err(RichamsterError::IllegalCurrencyPair(pair));
        };
        let RequestData(mut url, method) = Api::Exchange(ExchangeApi::Favourites).request_data();
        let path_segments: Vec<&str> = url.path_segments().unwrap().collect();
        let new_path = replace_placeholder(path_segments, market.id.to_string(), "{id}");
        url.set_path(new_path.as_str());

        let resp = self.send_request(url, method).await?;
        process_response!(resp, FavouritePairResponse)
    }

    pub async fn currencies_list(
        &self,
        token: Option<token::Token>,
    ) -> Result<Vec<CurrencyInfo>, RichamsterError> {
        let RequestData(mut url, method) = Api::Exchange(ExchangeApi::Currencies).request_data();
        if let Some(t) = token {
            url.query_pairs_mut()
                .append_pair("abbreviation", t.as_ref());
        }
        let resp = self.send_request(url, method).await?;
        process_response!(resp, Vec<CurrencyInfo>)
    }

    pub async fn markets_list(&self) -> Result<Vec<Market>, RichamsterError> {
        let RequestData(url, method) = Api::Exchange(ExchangeApi::Currencies).request_data();
        let resp = self.send_request(url, method).await?;
        process_response!(resp, Vec<Market>)
    }

    pub async fn order_book(&self, filter: OrderBookFilter) -> Result<OrdersBook, RichamsterError> {
        let RequestData(mut url, method) = Api::Exchange(ExchangeApi::OrderBook).request_data();
        let url = filter.compose_url(&mut url);
        Ok(self.send_request(url, method).await?.json().await?)
    }

    pub async fn orders_history(
        &self,
        filter: OrdersFilter,
    ) -> Result<OrdersHistory, RichamsterError> {
        let RequestData(mut url, method) = Api::Exchange(ExchangeApi::OrdersHistory).request_data();
        let url = filter.compose_url(&mut url);
        Ok(self.send_request(url, method).await?.json().await?)
    }

    pub async fn next_orders_history(&self, url: Url) -> Result<OrdersHistory, RichamsterError> {
        let response = send_request!(url, get);
        let string = response.text().await?;
        let orders_history: OrdersHistory = serde_json::from_str(&string)?;
        Ok(orders_history)
    }

    pub async fn destroy_user_order(&self, id: i32) -> Result<(), RichamsterError> {
        let RequestData(mut url, method) = Api::Exchange(ExchangeApi::DestroyOrder).request_data();
        url = url.join(id.to_string().as_str())?;

        let resp = self.send_request(url, method).await?;
        match resp.status() {
            StatusCode::NO_CONTENT => Ok(()),
            StatusCode::UNAUTHORIZED => Err(RichamsterError::UnauthorizedAccess),
            StatusCode::NOT_FOUND => Err(RichamsterError::OrderNotFound(id)),
            status => Err(RichamsterError::UnsupportedResponseCode(
                status,
                resp.text().await?,
            )),
        }
    }

    pub async fn user_orders(
        &self,
        filter: OrdersFilter,
    ) -> Result<OrdersHistory, RichamsterError> {
        let RequestData(mut url, method) = Api::Exchange(ExchangeApi::UserOrders).request_data();
        let url = filter.compose_url(&mut url);
        let resp = self.send_request(url, method).await?;
        process_response!(resp, OrdersHistory)
    }

    pub async fn create_order(&self, order: NewOrder) -> Result<NewOrder, RichamsterError> {
        let RequestData(url, method) = Api::Exchange(ExchangeApi::NewOrder).request_data();
        let resp = self
            .send_request_with_body(url, method, serde_json::to_string(&order)?)
            .await?;

        match resp.status() {
            StatusCode::CREATED => {
                let response_string = resp.text().await?;
                let response: NewOrder = serde_json::from_str(&response_string)?;
                Ok(response)
            }
            StatusCode::UNAUTHORIZED => Err(RichamsterError::UnauthorizedAccess),
            StatusCode::BAD_REQUEST => {
                let response_string = resp.text().await?;
                let response: NewOrderError = serde_json::from_str(&response_string)?;
                Err(RichamsterError::NewOrderError(response))
            }
            status => {
                let response_string = resp.text().await?;
                Err(RichamsterError::UnsupportedResponseCode(
                    status,
                    response_string,
                ))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_default_exchange() {
        let exchange: Exchange = Default::default();
        match exchange.auth_state {
            AuthState::Unauthorized => assert!(true),
            _ => assert!(false),
        }
    }
}
