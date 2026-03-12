use crate::api::token::CurrencyPair;
use crate::models::common::OrderType;
use chrono::{DateTime, Local};
use serde_derive::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use url::Url;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct CurrencyInfoResponse {
    pub success: bool,
    pub data: Vec<CurrencyInfo>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct CurrencyInfo {
    pub id: i32,
    pub abbreviation: String,
    pub title: String,
    pub icon: Url,
    pub precision: i32,
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
    pub data: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct FavouriteErrorResponse {
    pub detail: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct TickerResponse {
    pub success: bool,
    pub data: Vec<Ticker>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Ticker {
    pub pk: i32,
    pub pair: String,
    pub last_price: String,
    pub first_price: String,
    pub high_price: Option<String>,
    pub low_price: Option<String>,
    pub base_volume: Option<String>,
    pub quote_volume: Option<String>,
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
    pub pair: CurrencyPair,
    pub order_type: Option<OrderType>,
}

impl OrderBookFilter {
    pub fn new(pair: CurrencyPair) -> Self {
        Self {
            pair,
            order_type: None,
        }
    }

    pub fn order_type(mut self, order_type: OrderType) -> Self {
        self.order_type = Some(order_type);
        self
    }
}

impl OrderBookFilter {
    pub fn compose_url(&self, url: &mut Url) -> String {
        let mut url_mut = url.query_pairs_mut();
        url_mut.append_pair("pair", self.pair.to_string().as_str());
        if let Some(order_type) = &self.order_type {
            url_mut.append_pair("side", order_type.to_string().to_lowercase().as_str());
        }
        url_mut.finish().to_string()
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
    pub pk: i32,
    #[serde(deserialize_with = "crate::models::deserialize::string_timestamp_deserialize")]
    pub created_at: DateTime<Local>,
    #[serde(deserialize_with = "crate::models::deserialize::option_timestamp_deserialize")]
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
        let mut url = url.query_pairs_mut();
        if let Some(pair) = &self.pair {
            url.append_pair("pair", pair.to_string().as_str());
        }
        if let Some(ordering) = &self.ordering {
            url.append_pair("ordering", ordering);
        }
        if let Some(page_size) = &self.page_size {
            url.append_pair("page_size", page_size.to_string().as_str());
        }
        url.finish().to_string()
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
    #[serde(deserialize_with = "crate::models::deserialize::date_string_deserialize")]
    pub closed_at: Option<DateTime<Local>>,
    #[serde(rename = "type")]
    pub o_type: Option<OrderType>,
    #[serde(deserialize_with = "crate::models::deserialize::date_string_deserialize")]
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
    pub order_type: String,
    pub errors: Vec<OrderError>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct OrderError {
    pub code: String,
    pub detail: String,
    pub attr: String,
}

impl Display for NewOrderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "New order error [{}]", self.order_type)?;
        for error in &self.errors {
            write!(
                f,
                "\n[code: {}, detail: {}, attr: {}]",
                error.code, error.detail, error.attr
            )?;
        }
        Ok(())
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
    pub total_sum: MarketOrderTotal,
    pub in_orders: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(untagged)]
pub enum MarketOrderTotal {
    String(String),
    F64(f64),
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct MarketOrderCalculator {
    pub average_price: f64,
    pub covered: f64,
    pub total_sum: f64,
}

impl MarketOrderInfo {
    pub fn compose_url(&self, url: &mut Url) -> String {
        url.query_pairs_mut()
            .append_pair("currency_pair", self.currency_pair.to_string().as_str())
            .append_pair("type", self.order_type.to_string().as_str())
            .append_pair("amount", self.amount.as_str());
        url.to_string()
    }
}
