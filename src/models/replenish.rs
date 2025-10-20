use crate::models::common::Currency;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct P2PReplenish {
    pub amount: String,
    pub currency: Currency,
    pub finserver_channel: i32,
    pub bank_card: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct ReplenishInfo {
    pub address: String,
}
