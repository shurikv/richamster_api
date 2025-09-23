use crate::models::common::TransactionType::{
    Conversion, Dividends, NftAuction, OtcTransfer, Referral, Replenish, Staking, Transfer,
    Unknown, Withdrawal,
};
use chrono::{DateTime, FixedOffset, Local, TimeZone};
use serde::{Deserialize, Deserializer};
use serde_derive::Serialize;
use strum_macros::Display;

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

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Display)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum OrderType {
    Buying,
    Selling,
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
    Conversion,
    Staking,
    OtcTransfer,
    NftAuction,
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
            6 => Conversion,
            7 => Staking,
            8 => OtcTransfer,
            9 => NftAuction,
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
            Conversion => 6,
            Staking => 7,
            OtcTransfer => 8,
            NftAuction => 9,
            Unknown => -1,
        }
    }
}

pub fn timestamp_deserialize<'de, D>(deserializer: D) -> Result<DateTime<FixedOffset>, D::Error>
where
    D: Deserializer<'de>,
{
    let ts: f64 = Deserialize::deserialize(deserializer)?;
    let secs = ts.trunc() as i64;
    let nanos = ((ts.fract()) * 1_000_000_000.0).round() as u32;
    let dt = DateTime::from_timestamp(secs, nanos).unwrap();
    Ok(DateTime::from(dt))
}

pub fn string_timestamp_deserialize<'de, D>(
    deserializer: D,
) -> Result<DateTime<Local>, D::Error>
where
    D: Deserializer<'de>,
{
    let timestamp: Option<String> = Deserialize::deserialize(deserializer)?;
    if timestamp.is_none() {
        return Err(serde::de::Error::custom("Timestamp is None"));
    }
    let date_time = DateTime::from_timestamp(timestamp.unwrap().parse().unwrap(), 0).unwrap();
    let local_datetime = Local.from_utc_datetime(&date_time.naive_utc());
    Ok(local_datetime)
}

pub fn option_timestamp_deserialize<'de, D>(
    deserializer: D,
) -> Result<Option<DateTime<Local>>, D::Error>
where
    D: Deserializer<'de>,
{
    let timestamp: Option<String> = Deserialize::deserialize(deserializer)?;
    if timestamp.is_none() {
        return Ok(None);
    }
    let date_time = DateTime::from_timestamp(timestamp.unwrap().parse().unwrap(), 0).unwrap();
    let local_datetime = Local.from_utc_datetime(&date_time.naive_utc());
    Ok(Some(local_datetime))
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct CurrencyChannel {
    pub id: i32,
    #[serde(rename = "finserver_currency_name")]
    pub currency_name: String,
    pub blockchain_protocol: i32,
    #[serde(rename = "currency_channel_short_description")]
    pub short_description: String,
    #[serde(rename = "finserver_channel_fee")]
    pub channel_fee: Option<String>,
    pub network: String,
    pub is_p2p_oriented: bool,
    pub bank_cards: Vec<String>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn order_type_to_string() {
        let order_type = OrderType::Buying;
        assert_eq!(order_type.to_string(), "buying");
    }
}