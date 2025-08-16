use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Login {
    email: String,
    password: String,
}

impl Login {
    pub fn new(email: impl AsRef<str>, password: impl AsRef<str>) -> Self {
        Self {
            email: email.as_ref().to_owned(),
            password: password.as_ref().to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum LoginResponse {
    Jwt(String),
    RequiresTwoFactor(bool),
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponseError {
    #[serde(rename = "type")]
    pub type_field: String,
    pub errors: Vec<Error>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Error {
    pub code: String,
    pub detail: String,
    pub attr: Value,
}

impl Display for LoginResponseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct RegisterUser {
    pub username: String,
    pub email: String,
    pub password1: String,
    pub password2: String,
    pub not_usa_citizen: bool,
    pub is_licence_agree: bool,
    pub referrer: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct RegisterUserResponse {
    pub message: Message,
    pub username: String,
    pub email: String,
    pub not_usa_citizen: bool,
    pub referrer: String,
    pub token: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct RegisterUserError {
    pub username: Option<Vec<String>>,
    pub email: Option<Vec<String>>,
    pub referrer: Option<Vec<String>>,
    pub non_field_errors: Option<Vec<String>>,
}

impl Display for RegisterUserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Message {
    pub header: String,
    pub text: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct OtpLogin {
    pub otp_token: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct OtpLoginResponseError {
    #[serde(rename = "type")]
    pub error_type: String,
    pub errors: Vec<CommonError>
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct CommonError {
    pub code: String,
    pub detail: String,
    pub attr: String,
}

impl Display for OtpLoginResponseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum OtpLoginResponse {
    Jwt(String),
    Error(OtpLoginResponseError),
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct TokenData {
    pub access: String,
    pub refresh: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct RefreshToken {
    pub refresh: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct NonFieldsError {
    pub non_field_errors: Vec<String>,
}

impl Display for NonFieldsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
