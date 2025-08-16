use crate::api::ExchangeApi;
use crate::api::token::CurrencyPair;
use crate::api::{Api, RequestPath};
use crate::api::{RequestData, token};
use crate::errors::RichamsterError;
use crate::models::exchange::{
    CurrencyInfoResponse, CurrencyPairRestriction, FavouritePairResponse, MarketResponse, NewOrder,
    NewOrderError, OrderBookFilter, OrdersBook, OrdersFilter, OrdersHistory, TickerResponse,
};
use crate::richamster::common::{ApiKey, AuthState, HeaderCompose, JwtToken, SecretKey};
use crate::{process_response, send_request};
use percent_encoding::percent_decode_str;
use reqwest::StatusCode;
use secrecy::SecretBox;
use url::Url;

#[derive(Default)]
pub struct Exchange {
    auth_state: AuthState,
}

impl Exchange {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_jwt_token(token: String) -> Self {
        Self {
            auth_state: AuthState::JwtTokenAuth(JwtToken(SecretBox::new(Box::new(token)))),
        }
    }

    pub fn with_keys(api_key: String, secret_key: String) -> Self {
        Self {
            auth_state: AuthState::ApiSecretKeyAuth(
                ApiKey(SecretBox::new(Box::new(api_key))),
                SecretKey(SecretBox::new(Box::new(secret_key))),
            ),
        }
    }
}

impl Exchange {
    pub async fn restrictions_list(&self) -> Result<Vec<CurrencyPairRestriction>, RichamsterError> {
        let RequestData(url, method) = Api::Exchange(ExchangeApi::Restrictions).request_data();
        Ok(send_request!(url, method, self.auth_state).json().await?)
    }

    pub async fn ticker_list(
        &self,
        pair: Option<CurrencyPair>,
    ) -> Result<TickerResponse, RichamsterError> {
        let RequestData(mut url, method) = Api::Exchange(ExchangeApi::TickerList).request_data();
        if let Some(pair) = pair {
            url.query_pairs_mut()
                .append_pair("pair", pair.to_string().as_str());
        }
        Ok(send_request!(url, method, self.auth_state).json().await?)
    }

    pub async fn favourites_pair_toggle(
        &self,
        pair: CurrencyPair,
    ) -> Result<FavouritePairResponse, RichamsterError> {
        let market_list = self.markets_list().await?;
        let market = if let Some(m) = market_list
            .data
            .iter()
            .find(|m| m.abbreviation == pair.to_string())
        {
            m
        } else {
            return Err(RichamsterError::IllegalCurrencyPair(pair));
        };
        let RequestData(url, method) = Api::Exchange(ExchangeApi::Favourites).request_data();
        let url = percent_decode_str(url.to_string().as_str())
            .decode_utf8_lossy()
            .replace("{id}", market.id.to_string().as_str());

        let resp = send_request!(url, method, self.auth_state);
        process_response!(resp, FavouritePairResponse)
    }

    pub async fn currencies_list(
        &self,
        token: Option<token::Token>,
    ) -> Result<CurrencyInfoResponse, RichamsterError> {
        let RequestData(mut url, method) = Api::Exchange(ExchangeApi::Currencies).request_data();
        if let Some(t) = token {
            url.query_pairs_mut()
                .append_pair("abbreviation", t.as_ref());
        }
        let resp = send_request!(url, method, self.auth_state);
        process_response!(resp, CurrencyInfoResponse)
    }

    pub async fn markets_list(&self) -> Result<MarketResponse, RichamsterError> {
        let RequestData(url, method) = Api::Exchange(ExchangeApi::Markets).request_data();
        let resp = send_request!(url, method, self.auth_state);
        process_response!(resp, MarketResponse)
    }

    pub async fn order_book(&self, filter: OrderBookFilter) -> Result<OrdersBook, RichamsterError> {
        let RequestData(mut url, method) = Api::Exchange(ExchangeApi::OrderBook).request_data();
        let url = filter.compose_url(&mut url);
        Ok(send_request!(url, method, self.auth_state).json().await?)
    }

    pub async fn orders_history(
        &self,
        filter: OrdersFilter,
    ) -> Result<OrdersHistory, RichamsterError> {
        let RequestData(mut url, method) = Api::Exchange(ExchangeApi::OrdersHistory).request_data();
        let url = filter.compose_url(&mut url);
        let response = send_request!(url, method, self.auth_state);
        let string = response.text().await?;
        let response: OrdersHistory = serde_json::from_str(&string)?;
        Ok(response)
    }

    pub async fn next_orders_history(&self, url: Url) -> Result<OrdersHistory, RichamsterError> {
        let response = send_request!(url, reqwest::Method::GET, self.auth_state);
        let string = response.text().await?;
        let orders_history: OrdersHistory = serde_json::from_str(&string)?;
        Ok(orders_history)
    }

    pub async fn destroy_user_order(&self, id: i32) -> Result<(), RichamsterError> {
        let RequestData(mut url, method) = Api::Exchange(ExchangeApi::DestroyOrder).request_data();
        url = url.join(id.to_string().as_str())?;
        let resp = send_request!(url, method, self.auth_state);
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
        let resp = send_request!(url, method, self.auth_state);
        let string = resp.text().await?;
        println!("user_orders: {}", string);
        let response: OrdersHistory = serde_json::from_str(&string)?;
        Ok(response)
        // process_response!(resp, OrdersHistory)
    }

    pub async fn create_order(&self, order: NewOrder) -> Result<NewOrder, RichamsterError> {
        let RequestData(url, method) = Api::Exchange(ExchangeApi::NewOrder).request_data();
        let resp = send_request!(url, method, self.auth_state, serde_json::to_string(&order)?);

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
