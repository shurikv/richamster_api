use crate::api::token::CurrencyPair;
use crate::models::common::OrderType;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_derive::Serialize;
use std::fmt::{Display, Formatter};
use url::Url;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct CurrencyInfo {
    id: i32,
    abbreviation: String,
    title: String,
    icon: Url,
    precision: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Market {
    pub id: i32,
    pub is_favourite: bool,
    pub abbreviation: String,
    pub volume: f64,
    pub price_deviation: i32,
    pub last_price: f64,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct CurrencyPairRestriction {
    currency_pair: String,
    min_quantity: String,
    price_scale: i32,
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
pub struct Ticker {
    pk: i32,
    pair: String,
    last: String,
    first: String,
    high: Option<String>,
    low: Option<String>,
    volume: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Order {
    volume: String,
    unit_price: String,
    sum: String,
    side: OrderType,
    pair: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct OrdersBook {
    buying: Option<Vec<Order>>,
    selling: Option<Vec<Order>>,
}

pub struct OrderBookFilter {
    pair: Option<CurrencyPair>,
    order_type: Option<OrderType>,
}

impl OrderBookFilter {
    pub fn new(pair: Option<CurrencyPair>, order_type: Option<OrderType>) -> Self {
        Self { pair, order_type }
    }

    pub fn compose_url(&self) -> String {
        let mut result: Vec<String> = Vec::new();
        if let Some(pair) = &self.pair {
            result.push(format!("pair={}", pair));
        }
        if let Some(order_type) = &self.order_type {
            result.push(format!("side={}", order_type.to_string().to_lowercase()));
        }
        result.join("&")
    }
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
    #[serde(deserialize_with = "crate::models::common::timestamp_deserialize")]
    created_at: DateTime<Utc>,
    #[serde(deserialize_with = "crate::models::common::option_timestamp_deserialize")]
    closed_at: Option<DateTime<Utc>>,
    side: OrderType,
    volume: String,
    unit_price: String,
    sum: String,
    pair: String,
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

    pub fn compose_url(&self) -> String {
        let mut result: Vec<String> = Vec::new();
        if let Some(pair) = &self.pair {
            result.push(format!("pair={}", pair));
        }
        if let Some(ordering) = &self.ordering {
            result.push(format!("ordering={}", ordering));
        }
        if let Some(page_size) = &self.page_size {
            result.push(format!("page_size={}", page_size));
        }
        result.join("&")
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct NewOrder {
    pub pk: Option<i32>,
    pub order_type: Option<OrderType>,
    pub amount: String,
    pub unit_price: String,
    pub currency_pair: String,
    pub commission: Option<String>,
    pub closed_at: Option<String>,
    #[serde(rename = "type")]
    pub o_type: Option<OrderType>,
    pub created_at: Option<String>,
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
    amount: Option<Vec<String>>,
    unit_price: Option<Vec<String>>,
    currency_pair: Option<Vec<String>>,
    #[serde(rename = "type")]
    order_type: Option<Vec<String>>,
    non_field_errors: Option<Vec<String>>,
}

impl Display for NewOrderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct MarketOrderInfo {
    amount: String,
    currency_pair: String,
    #[serde(rename = "type")]
    order_type: OrderType,
    total: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct MarketOrderParameters {
    pub amount: String,
    pub currency_pair: String,
    pub order_type: OrderType,
    pub total: String,
}