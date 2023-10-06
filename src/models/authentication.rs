use serde_derive::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Login {
    username: String,
    password: String,
}

impl Login {
    pub fn new(username: impl AsRef<str>, password: impl AsRef<str>) -> Self {
        Self {
            username: username.as_ref().to_owned(),
            password: password.as_ref().to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum LoginResponse {
    Jwt(String),
    RequiresTwoFactor(bool),
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct LoginResponseError {
    pub username: Option<Vec<String>>,
    pub password: Option<Vec<String>>,
    pub non_field_errors: Option<Vec<String>>,
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
    pub otp_token: Vec<String>,
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
    #[serde(rename = "token")]
    pub jwt_token: String,
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
