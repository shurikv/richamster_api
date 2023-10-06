use serde_derive::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct WithdrawInfo {
    pub fee: f64,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct ReplenishInfo {
    pub address: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Withdraw {
    pub address: String,
    pub sum: String,
    pub fee: Option<String>,
    pub pin_code: String,
    pub minimum_confirmations: u8,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct WithdrawResponse {
    pub response_message: String,
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
