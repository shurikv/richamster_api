use std::fmt::{Display, Formatter};
use serde_derive::{Deserialize, Serialize};
use crate::models::common::CurrencyChannel;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct WithdrawInfoResponse {
    pub fee: f64,
    pub channels: Vec<CurrencyChannel>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct WithdrawData {
    pub address: String,
    pub sum: String,
    pub pin_code: String,
    pub finserver_channel_name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum WithdrawError {
    Fields(WithdrawFieldError),
    Detail(WithdrawDetailError),
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct WithdrawFieldError {
    pub address: Option<Vec<String>>,
    pub sum: Option<Vec<String>>,
    pub fee: Option<Vec<String>>,
    pub pin_code: Option<Vec<String>>,
    pub minimum_confirmations: Option<Vec<String>>,
    pub non_field_errors: Option<Vec<String>>,
}

impl Display for WithdrawError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct WithdrawDetailError {
    pub detail: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct WithdrawResponse {
    pub status: String,
}