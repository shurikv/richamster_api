use crate::api::token;
use crate::api::token::CurrencyPair;
use crate::api::ExchangeApi;
use crate::api::ExchangeApi::{Currencies, DestroyOrder, Favourites, UserOrders};
use crate::api::{Api, RequestPath};
use crate::errors::RichamsterError;
use crate::models::exchange::{
    CurrencyInfo, CurrencyPairRestriction, FavouritePairResponse, Market, NewOrder, NewOrderError,
    OrderBookFilter, OrdersBook, OrdersFilter, OrdersHistory, Ticker,
};
use crate::richamster::common::AuthState::Unauthorized;
use crate::richamster::common::{ApiKey, AuthState, HeaderCompose, JwtToken, SecretKey};
use crate::richamster::replace_placeholder;
use crate::{prepare_request, process_response, send_request};
use reqwest::StatusCode;
use secrecy::Secret;
use url::Url;

pub struct Exchange {
    auth_state: AuthState,
}

impl Default for Exchange {
    fn default() -> Self {
        Self {
            auth_state: Unauthorized,
        }
    }
}

impl Exchange {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_jwt_token(token: String) -> Self {
        Self {
            auth_state: AuthState::JwtTokenAuth(JwtToken(Secret::new(token))),
        }
    }

    pub fn with_keys(api_key: String, secret_key: String) -> Self {
        Self {
            auth_state: AuthState::ApiSecretKeyAuth(
                ApiKey(Secret::new(api_key)),
                SecretKey(Secret::new(secret_key)),
            ),
        }
    }
}

impl Exchange {
    pub async fn restrictions_list(&self) -> Result<Vec<CurrencyPairRestriction>, RichamsterError> {
        let response = send_request!(Api::Exchange(ExchangeApi::Restrictions).full_url(), get);
        let response = response.text().await?;
        let currency_restriction: Vec<CurrencyPairRestriction> = serde_json::from_str(&response)?;
        Ok(currency_restriction)
    }

    pub async fn ticker_list(
        &self,
        pair: Option<CurrencyPair>,
    ) -> Result<Vec<Ticker>, RichamsterError> {
        let mut url = Api::Exchange(ExchangeApi::TickerList).full_url();
        if let Some(pair) = pair {
            url = url.join(format!("?pair={}", pair).as_str())?;
        }
        let response = send_request!(url, get);
        let string = response.text().await?;
        let ticker_list: Vec<Ticker> = serde_json::from_str(&string)?;
        Ok(ticker_list)
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
        let mut url: Url = Api::Exchange(Favourites).full_url();
        let path_segments: Vec<&str> = url.path_segments().unwrap().collect();
        let new_path = replace_placeholder(path_segments, market.id.to_string(), "{id}");
        url.set_path(new_path.as_str());

        let resp = prepare_request!(url, post)
            .compose(&self.auth_state)
            .send()
            .await?;
        process_response!(resp, FavouritePairResponse)
    }

    pub async fn currencies_list(
        &self,
        token: Option<token::Token>,
    ) -> Result<Vec<CurrencyInfo>, RichamsterError> {
        let mut url = Api::Exchange(Currencies).full_url();
        if let Some(t) = token {
            url = url.join(
                format!("?abbreviation={}", <token::Token as Into<&str>>::into(t)).as_str(),
            )?;
        }
        let resp = prepare_request!(url, get)
            .compose(&self.auth_state)
            .send()
            .await?;
        process_response!(resp, Vec<CurrencyInfo>)
    }

    pub async fn markets_list(&self) -> Result<Vec<Market>, RichamsterError> {
        let resp = prepare_request!(Api::Exchange(ExchangeApi::Markets).full_url(), get)
            .compose(&self.auth_state)
            .send()
            .await?;
        process_response!(resp, Vec<Market>)
    }

    pub async fn order_book(&self, filter: OrderBookFilter) -> Result<OrdersBook, RichamsterError> {
        let mut url = Api::Exchange(ExchangeApi::OrderBook).full_url();
        let params = filter.compose_url();
        if !params.is_empty() {
            url = url.join(format!("?{}", params).as_str())?;
        }
        let response = send_request!(url, get);
        let order_book: OrdersBook = serde_json::from_str(&response.text().await?)?;
        Ok(order_book)
    }

    pub async fn orders_history(
        &self,
        filter: OrdersFilter,
    ) -> Result<OrdersHistory, RichamsterError> {
        let mut url = Api::Exchange(ExchangeApi::OrdersHistory).full_url();
        let params = filter.compose_url();
        if !params.is_empty() {
            url = url.join(format!("?{}", params).as_str())?;
        }
        let response = send_request!(url, get);
        let string = response.text().await?;
        let orders_history: OrdersHistory = serde_json::from_str(&string)?;
        Ok(orders_history)
    }

    pub async fn next_orders_history(&self, url: Url) -> Result<OrdersHistory, RichamsterError> {
        let response = send_request!(url, get);
        let string = response.text().await?;
        let orders_history: OrdersHistory = serde_json::from_str(&string)?;
        Ok(orders_history)
    }

    pub async fn destroy_user_order(&self, id: i32) -> Result<(), RichamsterError> {
        let mut url: Url = Api::Exchange(DestroyOrder).full_url();
        let path_segments: Vec<&str> = url.path_segments().unwrap().collect();
        let new_path = replace_placeholder(path_segments, id.to_string(), "{id}");
        url.set_path(new_path.as_str());

        let resp = prepare_request!(url, delete)
            .compose(&self.auth_state)
            .send()
            .await?;
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
        let mut url = Api::Exchange(UserOrders).full_url();
        let params = filter.compose_url();
        if !params.is_empty() {
            url = url.join(format!("?{}", params).as_str())?;
        }
        let resp = prepare_request!(url, get)
            .compose(&self.auth_state)
            .send()
            .await?;
        process_response!(resp, OrdersHistory)
    }

    pub async fn create_order(&self, order: NewOrder) -> Result<NewOrder, RichamsterError> {
        let payload = serde_json::to_string(&order)?;
        let resp = prepare_request!(
            Api::Exchange(ExchangeApi::NewOrder).full_url(),
            payload,
            post
        )
        .compose_with_payload(&self.auth_state, &payload)
        .send()
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
