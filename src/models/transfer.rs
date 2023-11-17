use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct TransferQuery {
    pub amount: String,
    pub currency: String,
    pub to: String,
    pub pin_code: String,
}
