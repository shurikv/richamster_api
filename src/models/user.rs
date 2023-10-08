use crate::api::token::{CurrencyPair, Token};
use crate::models::common::{Currency, OrderType, TransactionStatus, TransactionType};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Deserializer};
use serde_derive::Serialize;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(untagged)]
pub enum ActiveBalanceType {
    String(String),
    Float(f64),
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct UserBalance {
    pub currency: Currency,
    pub balance: String,
    pub active_balance: ActiveBalanceType,
    pub in_orders: String,
    pub in_auctions: String,
    pub in_krb: String,
    pub in_btc: f64,
    pub in_grn: String,
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
    pub closed_at: String,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub unit_price: String,
    pub volume: String,
    pub sum: String,
    pub pair: String,
}

pub struct TransactionsFilter {
    pub currency: Option<Token>,
    pub transaction_type: Option<TransactionType>,
    pub closed_at_gte: Option<i32>,
    pub closed_at_lte: Option<i32>,
}

impl TransactionsFilter {
    pub fn compose_url(&self) -> String {
        let mut result: Vec<String> = Vec::new();
        if let Some(token) = &self.currency {
            result.push(format!("currency={}", token.as_ref()));
        }
        if let Some(transaction_type) = &self.transaction_type {
            result.push(format!(
                "type={}",
                <TransactionType as Into<i32>>::into(*transaction_type)
            ));
        }
        if let Some(closed_at_gte) = &self.closed_at_gte {
            result.push(format!("closed_at__gte={}", closed_at_gte));
        }
        if let Some(closed_at_lte) = &self.closed_at_lte {
            result.push(format!("closed_at__lte={}", closed_at_lte));
        }
        result.join("&")
    }
}

pub struct OrdersFilter {
    pub pair: Option<CurrencyPair>,
    pub order_type: Option<OrderType>,
    pub closed_at_gte: Option<i32>,
    pub closed_at_lte: Option<i32>,
}

impl OrdersFilter {
    pub fn compose_url(&self) -> String {
        let mut result: Vec<String> = Vec::new();
        if let Some(pair) = &self.pair {
            result.push(format!("pair={}", pair));
        }
        if let Some(order_type) = &self.order_type {
            result.push(format!("side={}", order_type));
        }
        if let Some(closed_at_gte) = &self.closed_at_gte {
            result.push(format!("closed_at__gte={}", closed_at_gte));
        }
        if let Some(closed_at_lte) = &self.closed_at_lte {
            result.push(format!("closed_at__lte={}", closed_at_lte));
        }
        result.join("&")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn transactions_filter_compose() {
        let tr_par = TransactionsFilter {
            currency: Some(Token::ADA),
            transaction_type: Some(TransactionType::Transfer),
            closed_at_gte: None,
            closed_at_lte: None,
        };
        let str = tr_par.compose_url();
        assert_eq!(str, "currency=ADA&type=5".to_owned());
    }

    #[test]
    fn empty_transactions_filter_compose() {
        let tr_par = TransactionsFilter {
            currency: None,
            transaction_type: None,
            closed_at_gte: None,
            closed_at_lte: None,
        };
        let str = tr_par.compose_url();
        assert!(str.is_empty());
    }

    #[test]
    fn one_param_transactions_parameters_compose() {
        let tr_par = TransactionsFilter {
            currency: Some(Token::ADA),
            transaction_type: None,
            closed_at_gte: None,
            closed_at_lte: None,
        };
        let str = tr_par.compose_url();
        assert_eq!(str, "currency=ADA".to_owned());
    }
}
