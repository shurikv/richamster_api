use crate::api::token::CurrencyPair;
use crate::models::authentication::{
    LoginResponseError, NonFieldsError, OtpLoginResponseError, RegisterUserError,
};
use crate::models::exchange::NewOrderError;
use crate::models::feedback::ContactUsError;
use crate::models::payments::WithdrawError;
use reqwest::StatusCode;
use thiserror::Error;
use url::ParseError;

#[derive(Error, Debug)]
pub enum RichamsterError {
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Unauthorized access")]
    UnauthorizedAccess,
    #[error("Invalid authorization type")]
    InvalidAuthorizationType,
    #[error("Login response error: {0}")]
    Login(LoginResponseError),
    #[error("Two factor response error: {0}")]
    Otp(OtpLoginResponseError),
    #[error("Serde error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Register user error: {0}")]
    Register(RegisterUserError),
    #[error("Unsupported response code: {0:?}; details: {1}")]
    UnsupportedResponseCode(StatusCode, String),
    #[error("Contact us error: {0}")]
    ContactUs(ContactUsError),
    #[error("Invalid currency pair: {0}")]
    IllegalCurrencyPair(CurrencyPair),
    #[error("Service unavailable")]
    ServiceUnavailable,
    #[error("Url parse error: {0}")]
    UrlParse(#[from] ParseError),
    #[error("Invalid JWT token: {0}")]
    InvalidJwtToken(NonFieldsError),
    #[error("Order {0} not found")]
    OrderNotFound(i32),
    #[error("Withdraw error: {0}")]
    WithdrawError(WithdrawError),
    #[error("Creation order error: {0}")]
    NewOrderError(NewOrderError),
}
