use crate::api::token::CurrencyPair;
use crate::models::common::OrderType;
use chrono::{DateTime, Local};
use serde::Deserialize;
use serde_derive::Serialize;
use std::fmt::{Display, Formatter};
use url::Url;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct CurrencyInfoResponse {
    pub success: bool,
    pub data: Vec<CurrencyInfo>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct CurrencyInfo {
    id: i32,
    abbreviation: String,
    title: String,
    icon: Url,
    precision: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct MarketResponse {
    pub success: bool,
    pub data: Vec<Market>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Market {
    pub id: i32,
    pub is_favourite: bool,
    pub abbreviation: String,
    pub volume: f64,
    pub price_deviation: i32,
    pub last_price: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct CurrencyPairRestrictionResponse {
    pub success: bool,
    pub data: Vec<CurrencyPairRestriction>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct CurrencyPairRestriction {
    pub id: i32,
    pub currency_pair: String,
    pub min_quantity: String,
    pub price_scale: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct FavouritePairResponse {
    data: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct FavouriteErrorResponse {
    detail: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct TickerResponse {
    pub success: bool,
    pub data: Vec<Ticker>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Ticker {
    pk: i32,
    pair: String,
    last_price: String,
    first_price: String,
    high_price: Option<String>,
    low_price: Option<String>,
    base_volume: Option<String>,
    quote_volume: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct OrderResponse {
    pub success: bool,
    pub data: Vec<Order>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Order {
    pub volume: String,
    pub unit_price: String,
    pub sum: String,
    pub side: OrderType,
    pub pair: String,
}

impl Display for Order {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Order [pair: {}, side: {:?}, volume: {}, unit_price: {}, sum: {}]",
            self.pair, self.side, self.volume, self.unit_price, self.sum
        )
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct OrdersBook {
    pub buying: Option<Vec<Order>>,
    pub selling: Option<Vec<Order>>,
}

pub struct OrderBookFilter {
    pub pair: Option<CurrencyPair>,
    pub order_type: Option<OrderType>,
}

#[derive(Default)]
pub struct OrderBookFilterBuilder {
    pub pair: Option<CurrencyPair>,
    pub order_type: Option<OrderType>,
}

impl OrderBookFilterBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pair(mut self, pair: CurrencyPair) -> Self {
        self.pair = Some(pair);
        self
    }

    pub fn order_type(mut self, order_type: OrderType) -> Self {
        self.order_type = Some(order_type);
        self
    }

    pub fn build(self) -> OrderBookFilter {
        OrderBookFilter {
            pair: self.pair,
            order_type: self.order_type,
        }
    }
}

impl OrderBookFilter {
    pub fn compose_url(&self, url: &mut Url) -> String {
        if let Some(pair) = &self.pair {
            url.query_pairs_mut()
                .append_pair("pair", pair.to_string().as_str());
        }
        if let Some(order_type) = &self.order_type {
            url.query_pairs_mut()
                .append_pair("side", order_type.to_string().to_lowercase().as_str());
        }
        url.to_string()
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct OrdersHistoryResponse {
    pub success: bool,
    pub data: Vec<OrderHistoryRecord>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct OrdersHistory {
    pub next: Option<Url>,
    pub previous: Option<Url>,
    pub results: Vec<OrderHistoryRecord>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct OrderHistoryRecord {
    pk: i32,
    #[serde(deserialize_with = "crate::models::common::string_timestamp_deserialize")]
    pub created_at: DateTime<Local>,
    #[serde(deserialize_with = "crate::models::common::option_timestamp_deserialize")]
    pub closed_at: Option<DateTime<Local>>,
    pub side: OrderType,
    pub volume: String,
    pub unit_price: String,
    pub sum: String,
    pub pair: String,
}

pub struct OrdersFilter {
    pair: Option<CurrencyPair>,
    ordering: Option<String>,
    page_size: Option<i32>,
}

impl OrdersFilter {
    pub fn new(
        pair: Option<CurrencyPair>,
        ordering: Option<String>,
        page_size: Option<i32>,
    ) -> Self {
        Self {
            pair,
            ordering,
            page_size,
        }
    }

    pub fn compose_url(&self, url: &mut Url) -> String {
        if let Some(pair) = &self.pair {
            url.query_pairs_mut()
                .append_pair("pair", pair.to_string().as_str());
        }
        if let Some(ordering) = &self.ordering {
            url.query_pairs_mut().append_pair("ordering", ordering);
        }
        if let Some(page_size) = &self.page_size {
            url.query_pairs_mut()
                .append_pair("page_size", page_size.to_string().as_str());
        }
        url.to_string()
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct NewOrderResponse {
    pub success: bool,
    pub data: NewOrder,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct NewOrder {
    pub pk: Option<i32>,
    pub order_type: Option<OrderType>,
    pub amount: String,
    pub unit_price: String,
    pub currency_pair: String,
    pub commission: Option<String>,
    #[serde(deserialize_with = "crate::models::common::date_string_deserialize")]
    pub closed_at: Option<DateTime<Local>>,
    #[serde(rename = "type")]
    pub o_type: Option<OrderType>,
    #[serde(deserialize_with = "crate::models::common::date_string_deserialize")]
    pub created_at: Option<DateTime<Local>>,
    pub is_partial: Option<bool>,
}

impl NewOrder {
    pub fn new(
        amount: String,
        unit_price: String,
        currency_pair: CurrencyPair,
        order_type: OrderType,
    ) -> Self {
        Self {
            pk: None,
            order_type: None,
            amount,
            unit_price,
            currency_pair: currency_pair.to_string(),
            commission: None,
            closed_at: None,
            o_type: Some(order_type),
            created_at: None,
            is_partial: None,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct NewOrderError {
    #[serde(rename = "type")]
    order_type: String,
    errors: Vec<OrderError>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct OrderError {
    code: String,
    detail: String,
    attr: String,
}

impl Display for NewOrderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct MarketOrderInfo {
    pub amount: String,
    pub currency_pair: i32,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct MarketOrderResponse {
    pub total_sum: f32,
    pub in_orders: i32,
}
