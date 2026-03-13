use crate::api::ExchangeApi;
use crate::api::token::CurrencyPair;
use crate::api::{Api, RequestPath};
use crate::api::{RequestData, token};
use crate::errors::RichamsterError;
use crate::models::common::OrderType;
use crate::models::exchange::{
    CurrencyInfoResponse, CurrencyPairRestriction, FavouritePairResponse, Market,
    MarketOrderCalculator, MarketOrderInfo, MarketOrderResponse, NewOrder, NewOrderError,
    OrderBookFilter, OrdersBook, OrdersFilter, OrdersHistory, TickerResponse,
};
use crate::richamster::common::{
    ApiKey, AuthState, JwtToken, SecretKey, process_response, send_request_with_auth,
    send_request_with_body_and_auth,
};
use percent_encoding::percent_decode_str;
use reqwest::StatusCode;
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
            auth_state: AuthState::JwtTokenAuth(JwtToken::new(token)),
        }
    }

    pub fn with_keys(api_key: String, secret_key: String) -> Self {
        Self {
            auth_state: AuthState::ApiSecretKeyAuth(
                ApiKey::new(api_key),
                SecretKey::new(secret_key),
            ),
        }
    }
}

impl Exchange {
    pub async fn restrictions_list(&self) -> Result<Vec<CurrencyPairRestriction>, RichamsterError> {
        let RequestData(url, method) = Api::Exchange(ExchangeApi::Restrictions).request_data();
        Ok(send_request_with_auth(url, method, &self.auth_state)
            .await?
            .json()
            .await?)
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
        Ok(send_request_with_auth(url, method, &self.auth_state)
            .await?
            .json()
            .await?)
    }

    async fn find_market(&self, pair: &CurrencyPair) -> Result<Market, RichamsterError> {
        let market_list = self.markets_list().await?;
        if let Some(market) = market_list
            .iter()
            .find(|m| m.abbreviation == pair.to_string())
        {
            return Ok(market.clone());
        }
        Err(RichamsterError::IllegalCurrencyPair(*pair))
    }

    pub async fn favourites_pair_toggle(
        &self,
        pair: CurrencyPair,
    ) -> Result<FavouritePairResponse, RichamsterError> {
        let market = self.find_market(&pair).await?;
        let RequestData(url, method) = Api::Exchange(ExchangeApi::Favourites).request_data();
        let url = percent_decode_str(url.to_string().as_str())
            .decode_utf8_lossy()
            .replace("{id}", market.id.to_string().as_str());

        let resp = send_request_with_auth(url, method, &self.auth_state).await?;
        process_response(resp).await
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
        let resp = send_request_with_auth(url, method, &self.auth_state).await?;
        process_response(resp).await
    }

    pub async fn markets_list(&self) -> Result<Vec<Market>, RichamsterError> {
        let RequestData(url, method) = Api::Exchange(ExchangeApi::Markets).request_data();
        let resp = send_request_with_auth(url, method, &self.auth_state).await?;
        process_response(resp).await
    }

    pub async fn order_book(&self, filter: OrderBookFilter) -> Result<OrdersBook, RichamsterError> {
        let RequestData(mut url, method) = Api::Exchange(ExchangeApi::OrderBook).request_data();
        let url = filter.compose_url(&mut url);
        Ok(send_request_with_auth(url, method, &self.auth_state)
            .await?
            .json()
            .await?)
    }

    pub async fn orders_history(
        &self,
        filter: OrdersFilter,
    ) -> Result<OrdersHistory, RichamsterError> {
        let RequestData(mut url, method) = Api::Exchange(ExchangeApi::OrdersHistory).request_data();
        let url = filter.compose_url(&mut url);
        let response = send_request_with_auth(url, method, &self.auth_state).await?;
        let string = response.text().await?;
        Ok(serde_json::from_str(&string)?)
    }

    pub async fn next_orders_history(&self, url: Url) -> Result<OrdersHistory, RichamsterError> {
        let response = send_request_with_auth(url, reqwest::Method::GET, &self.auth_state).await?;
        let string = response.text().await?;
        Ok(serde_json::from_str(&string)?)
    }

    pub async fn destroy_user_order(&self, id: i32) -> Result<(), RichamsterError> {
        let RequestData(mut url, method) = Api::Exchange(ExchangeApi::DestroyOrder).request_data();
        url = url.join(id.to_string().as_str())?;
        let resp = send_request_with_auth(url, method, &self.auth_state).await?;
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
        let resp = send_request_with_auth(url, method, &self.auth_state).await?;
        let string = resp.text().await?;
        Ok(serde_json::from_str(&string)?)
    }

    pub async fn create_order(&self, order: NewOrder) -> Result<NewOrder, RichamsterError> {
        let RequestData(url, method) = Api::Exchange(ExchangeApi::NewOrder).request_data();
        let resp = send_request_with_body_and_auth(
            url,
            method,
            serde_json::to_string(&order)?,
            &self.auth_state,
        )
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

    pub async fn calculate_market_order(
        &self,
        pair: CurrencyPair,
        amount: f64,
        order_type: OrderType,
    ) -> Result<MarketOrderCalculator, RichamsterError> {
        let market = self.find_market(&pair).await?;
        let RequestData(mut url, method) =
            Api::Exchange(ExchangeApi::CalculateMarketOrder).request_data();
        let market_order = MarketOrderInfo {
            amount: amount.to_string(),
            currency_pair: market.id,
            order_type,
            total: None,
        };
        market_order.compose_url(&mut url);
        let resp = send_request_with_auth(url, method, &self.auth_state).await?;
        match resp.status() {
            StatusCode::CREATED | StatusCode::OK => {
                let response_string = resp.text().await?;
                let response: MarketOrderCalculator = serde_json::from_str(&response_string)?;
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

    pub async fn execute_market_order(
        &self,
        pair: CurrencyPair,
        amount: f64,
        order_type: OrderType,
        total: Option<f64>,
    ) -> Result<MarketOrderResponse, RichamsterError> {
        let market = self.find_market(&pair).await?;
        let total = total.map(|t| t.to_string());
        let RequestData(url, method) =
            Api::Exchange(ExchangeApi::ExecuteMarketOrder).request_data();
        let market_order = MarketOrderInfo {
            amount: amount.to_string(),
            currency_pair: market.id,
            order_type,
            total,
        };
        let resp = send_request_with_body_and_auth(
            url,
            method,
            serde_json::to_string(&market_order)?,
            &self.auth_state,
        )
        .await?;
        match resp.status() {
            StatusCode::CREATED | StatusCode::OK => {
                let response_string = resp.text().await?;
                let response: MarketOrderResponse = serde_json::from_str(&response_string)?;
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
        assert!(matches!(exchange.auth_state, AuthState::Unauthorized))
    }
}
