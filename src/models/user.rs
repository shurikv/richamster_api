use crate::api::token::{CurrencyPair, Token};
use crate::models::common::{Currency, OrderType, TransactionStatus, TransactionType};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Deserializer};
use serde_derive::Serialize;
use std::fmt::{Display, Formatter};
use url::Url;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(untagged)]
pub enum ActiveBalanceType {
    String(String),
    Float(f64),
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct UserBalanceResponse {
    pub success: bool,
    pub data: Vec<UserBalance>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct UserBalance {
    pub currency: Currency,
    pub balance: String,
    pub active_balance: ActiveBalanceType,
    pub in_orders: String,
    pub in_usdt: String,
    pub in_btc: String,
    pub in_grn: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct UserDetailResponse {
    pub success: bool,
    pub data: UserDetail,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct UserDetail {
    pub username: String,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub dices: Option<i32>,
    pub role: Option<String>,
    pub phone: Option<String>,
    pub email: String,
    pub fee: Option<f32>,
}

impl Display for UserDetail {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Username: {:?}", self.username)?;
        if let Some(first_name) = &self.first_name {
            writeln!(f, "First name: {}", first_name)?;
        }
        if let Some(middle_name) = &self.middle_name {
            writeln!(f, "Middle name: {}", middle_name)?;
        }
        if let Some(last_name) = &self.last_name {
            writeln!(f, "Last name: {}", last_name)?;
        }
        if let Some(dices) = &self.dices {
            writeln!(f, "Dices: {}", dices)?;
        }
        if let Some(role) = &self.role {
            writeln!(f, "Role: {}", role)?;
        }
        if let Some(phone) = &self.phone {
            writeln!(f, "Phone: {}", phone)?;
        }
        writeln!(f, "Email: {}", &self.email)?;
        if let Some(fee) = &self.fee {
            writeln!(f, "Fee: {}", fee)?;
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct UserTransaction {
    #[serde(deserialize_with = "crate::models::common::timestamp_deserialize")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(deserialize_with = "crate::models::common::timestamp_deserialize")]
    pub closed_at: DateTime<FixedOffset>,
    pub status: TransactionStatus,
    #[serde(rename = "type")]
    #[serde(deserialize_with = "transaction_type_deserialize")]
    pub transaction_type: TransactionType,
    pub currency: String,
    pub sum: String,
    pub fee: String,
    pub balance: String,
    pub hash: String,
}

fn transaction_type_deserialize<'de, D>(deserializer: D) -> Result<TransactionType, D::Error>
where
    D: Deserializer<'de>,
{
    let transaction_type: i32 = Deserialize::deserialize(deserializer)?;
    let tr_type: TransactionType = transaction_type.into();
    Ok(tr_type)
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct UserOrder {
    pub pk: i32,
    #[serde(deserialize_with = "crate::models::common::timestamp_deserialize")]
    pub closed_at: DateTime<FixedOffset>,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub unit_price: String,
    pub volume: String,
    pub sum: String,
    pub pair: String,
    pub closed_type: OrderType,
    pub source: Option<String>,
}

pub struct TransactionsFilter {
    pub currency: Option<Token>,
    pub transaction_type: Option<TransactionType>,
    pub closed_at_gte: Option<i32>,
    pub closed_at_lte: Option<i32>,
}
impl TransactionsFilter {
    pub fn compose_url(&self, url: &mut Url) -> String {
        if let Some(token) = &self.currency {
            url.query_pairs_mut()
                .append_pair("currency", token.as_ref());
        }
        if let Some(transaction_type) = &self.transaction_type {
            let tr_type: i32 = (*transaction_type).into();
            url.query_pairs_mut()
                .append_pair("type", tr_type.to_string().as_str());
        }
        if let Some(closed_at_gte) = &self.closed_at_gte {
            url.query_pairs_mut()
                .append_pair("closed_at__gte", closed_at_gte.to_string().as_str());
        }
        if let Some(closed_at_lte) = &self.closed_at_lte {
            url.query_pairs_mut()
                .append_pair("closed_at__lte", closed_at_lte.to_string().as_str());
        }
        url.to_string()
    }
}

#[derive(Default)]
pub struct UserOrdersFilter {
    pub pair: Option<CurrencyPair>,
    pub order_type: Option<OrderType>,
    pub closed_at_gte: Option<i32>,
    pub closed_at_lte: Option<i32>,
}

impl UserOrdersFilter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pair(mut self, pair: CurrencyPair) -> Self {
        self.pair = Some(pair);
        self
    }

    pub fn order_type(self, order_type: OrderType) -> Self {
        Self {
            pair: self.pair,
            order_type: Some(order_type),
            closed_at_gte: self.closed_at_gte,
            closed_at_lte: self.closed_at_lte,
        }
    }

    pub fn closed_at_gte(self, closed_at_gte: i32) -> Self {
        Self {
            pair: self.pair,
            order_type: self.order_type,
            closed_at_gte: Some(closed_at_gte),
            closed_at_lte: self.closed_at_lte,
        }
    }

    pub fn closed_at_lte(self, closed_at_lte: i32) -> Self {
        Self {
            pair: self.pair,
            order_type: self.order_type,
            closed_at_gte: self.closed_at_gte,
            closed_at_lte: Some(closed_at_lte),
        }
    }

    pub fn compose_url(&self, url: &mut Url) -> String {
        if let Some(pair) = &self.pair {
            url.query_pairs_mut()
                .append_pair("pair", pair.to_string().as_str());
        }
        if let Some(order_type) = &self.order_type {
            url.query_pairs_mut()
                .append_pair("side", order_type.to_string().as_str());
        }
        if let Some(closed_at_gte) = &self.closed_at_gte {
            url.query_pairs_mut()
                .append_pair("closed_at__gte", closed_at_gte.to_string().as_str());
        }
        if let Some(closed_at_lte) = &self.closed_at_lte {
            url.query_pairs_mut()
                .append_pair("closed_at__lte", closed_at_lte.to_string().as_str());
        }
        url.to_string()
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct TransferQuery {
    pub amount: String,
    pub currency: Token,
    pub to: String,
    pub pin_code: String,
}

impl TransferQuery {
    pub fn new(amount: f64, currency: Token, to: String, pin_code: String) -> Self {
        Self {
            amount: amount.to_string(),
            currency,
            to,
            pin_code,
        }
    }
}
