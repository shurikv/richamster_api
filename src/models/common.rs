use crate::models::common::TransactionType::{
    Dividends, Referral, Replenish, Transfer, Unknown, Withdrawal,
};
use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Deserializer};
use serde_derive::Serialize;
use std::fmt;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Currency {
    pub id: i32,
    pub abbreviation: String,
    pub is_fiat: bool,
    pub is_auction_currency: bool,
    pub is_market: bool,
    pub precision: i32,
    pub can_input: bool,
    pub can_output: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum OrderType {
    Buying,
    Selling,
}

impl fmt::Display for OrderType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TransactionStatus {
    Draft,
    Confirmed,
    Failed,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Replenish,
    Withdrawal,
    Dividends,
    Referral,
    Transfer,
    Unknown,
}

impl From<i32> for TransactionType {
    fn from(value: i32) -> Self {
        match value {
            1 => Replenish,
            2 => Withdrawal,
            3 => Dividends,
            4 => Referral,
            5 => Transfer,
            _ => Unknown,
        }
    }
}

impl From<TransactionType> for i32 {
    fn from(value: TransactionType) -> Self {
        match value {
            Replenish => 1,
            Withdrawal => 2,
            Dividends => 3,
            Referral => 4,
            Transfer => 5,
            Unknown => -1,
        }
    }
}

pub fn timestamp_deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let timestamp: String = Deserialize::deserialize(deserializer)?;
    let datetime = Utc
        .datetime_from_str(&timestamp, "%s")
        .map_err(serde::de::Error::custom)?;
    Ok(datetime)
}

pub fn option_timestamp_deserialize<'de, D>(
    deserializer: D,
) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let timestamp: Option<String> = Deserialize::deserialize(deserializer)?;
    if timestamp.is_none() {
        return Ok(None);
    }
    let datetime = Utc
        .datetime_from_str(&timestamp.unwrap(), "%s")
        .map_err(serde::de::Error::custom)?;
    Ok(Some(datetime))
}
