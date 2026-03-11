use crate::models::common::CurrencyChannel;
use serde_derive::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

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
        match self {
            WithdrawError::Fields(withdraw_field_error) => {
                write!(f, "Withdraw field error")?;
                for (field, errors) in [
                    ("address", withdraw_field_error.address.as_ref()),
                    ("sum", withdraw_field_error.sum.as_ref()),
                    ("fee", withdraw_field_error.fee.as_ref()),
                    ("pin_code", withdraw_field_error.pin_code.as_ref()),
                    (
                        "minimum_confirmations",
                        withdraw_field_error.minimum_confirmations.as_ref(),
                    ),
                    (
                        "non_field_errors",
                        withdraw_field_error.non_field_errors.as_ref(),
                    ),
                ] {
                    if let Some(errors) = errors {
                        for error in errors {
                            write!(f, "\n{}: {}", field, error)?;
                        }
                    }
                }
                Ok(())
            }
            WithdrawError::Detail(withdraw_detail_error) => {
                write!(f, "Withdraw error [{}]", withdraw_detail_error.detail)
            }
        }
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
