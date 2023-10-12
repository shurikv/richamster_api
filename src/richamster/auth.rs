use crate::api::{Api, AuthenticationApi, RequestData, RequestPath};
use crate::errors::RichamsterError;
use crate::models::authentication::LoginResponse::{Jwt, RequiresTwoFactor};
use crate::models::authentication::{
    Login, LoginResponse, LoginResponseError, NonFieldsError, OtpLogin, OtpLoginResponse,
    OtpLoginResponseError, RegisterUser, RegisterUserError, RegisterUserResponse, TokenData,
};
use reqwest::{Client, IntoUrl, Method, Response, StatusCode};
use serde::Serialize;

pub struct Auth;

impl Auth {
    async fn send_request(
        url: impl IntoUrl,
        method: Method,
        body: impl Serialize,
    ) -> Result<Response, reqwest::Error> {
        Client::new().request(method, url).json(&body).send().await
    }

    pub async fn login(
        username: impl AsRef<str>,
        password: impl AsRef<str>,
    ) -> Result<LoginResponse, RichamsterError> {
        let login = Login::new(username.as_ref(), password.as_ref());
        let RequestData(url, method) = Api::Authentication(AuthenticationApi::Login).request_data();
        let resp = Self::send_request(url, method, login).await?;
        match resp.status() {
            StatusCode::OK => Ok(RequiresTwoFactor(true)),
            StatusCode::CREATED => {
                let token: TokenData = serde_json::from_str(&resp.text().await?)?;
                Ok(Jwt(token.jwt_token))
            }
            StatusCode::SERVICE_UNAVAILABLE => Err(RichamsterError::ServiceUnavailable),
            StatusCode::BAD_REQUEST => {
                let error: LoginResponseError = serde_json::from_str(&resp.text().await?)?;
                Err(RichamsterError::Login(error))
            }
            status => Err(RichamsterError::UnsupportedResponseCode(
                status,
                resp.text().await?,
            )),
        }
    }

    pub async fn register_user(
        user_creation: RegisterUser,
    ) -> Result<RegisterUserResponse, RichamsterError> {
        let RequestData(url, method) =
            Api::Authentication(AuthenticationApi::Register).request_data();
        let resp = Self::send_request(url, method, user_creation).await?;

        match resp.status() {
            StatusCode::CREATED => {
                let response: RegisterUserResponse = serde_json::from_str(&resp.text().await?)?;
                Ok(response)
            }
            _ => {
                let error: RegisterUserError = serde_json::from_str(&resp.text().await?)?;
                Err(RichamsterError::Register(error))
            }
        }
    }

    pub async fn two_factor_login(otp_token: String) -> Result<OtpLoginResponse, RichamsterError> {
        let RequestData(url, method) =
            Api::Authentication(AuthenticationApi::TwoFactorLogin).request_data();
        let resp = Self::send_request(url, method, OtpLogin { otp_token }).await?;
        if resp.status() == StatusCode::CREATED {
            let token: TokenData = serde_json::from_str(&resp.text().await?)?;
            Ok(OtpLoginResponse::Jwt(token.jwt_token))
        } else {
            let error: OtpLoginResponseError = serde_json::from_str(&resp.text().await?)?;
            Err(RichamsterError::Otp(error))
        }
    }

    pub async fn verify_token(jwt_token: String) -> Result<TokenData, RichamsterError> {
        let RequestData(url, method) =
            Api::Authentication(AuthenticationApi::VerifyToken).request_data();
        let resp = Self::send_request(url, method, TokenData { jwt_token }).await?;
        match resp.status() {
            StatusCode::OK => {
                let response: TokenData = serde_json::from_str(&resp.text().await?)?;
                Ok(response)
            }
            StatusCode::BAD_REQUEST => {
                let response: NonFieldsError = serde_json::from_str(&resp.text().await?)?;
                Err(RichamsterError::InvalidJwtToken(response))
            }
            status => {
                let response_text = resp.text().await?;
                Err(RichamsterError::UnsupportedResponseCode(
                    status,
                    response_text,
                ))
            }
        }
    }

    pub async fn refresh_token(jwt_token: String) -> Result<TokenData, RichamsterError> {
        let RequestData(url, method) =
            Api::Authentication(AuthenticationApi::VerifyToken).request_data();
        let resp = Self::send_request(url, method, TokenData { jwt_token }).await?;
        match resp.status() {
            StatusCode::OK => {
                let response: TokenData = serde_json::from_str(&resp.text().await?)?;
                Ok(response)
            }
            StatusCode::BAD_REQUEST => {
                let response: NonFieldsError = serde_json::from_str(&resp.text().await?)?;
                Err(RichamsterError::InvalidJwtToken(response))
            }
            status => Err(RichamsterError::UnsupportedResponseCode(
                status,
                resp.text().await?,
            )),
        }
    }
}
