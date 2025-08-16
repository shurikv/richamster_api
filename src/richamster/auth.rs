use crate::api::{Api, AuthenticationApi, RequestData, RequestPath};
use crate::errors::RichamsterError;
use crate::models::auth::LoginResponse::{Jwt, RequiresTwoFactor};
use crate::models::auth::{
    Login, LoginResponse, LoginResponseError, NonFieldsError, OtpLogin, OtpLoginResponse,
    OtpLoginResponseError, RefreshToken, RegisterUser, RegisterUserError, RegisterUserResponse,
    TokenData,
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
        email: impl AsRef<str>,
        password: impl AsRef<str>,
    ) -> Result<LoginResponse, RichamsterError> {
        let login = Login::new(email.as_ref(), password.as_ref());
        let RequestData(url, method) = Api::Authentication(AuthenticationApi::Login).request_data();
        let resp = Self::send_request(url, method, login).await?;
        match resp.status() {
            StatusCode::OK => Ok(RequiresTwoFactor(true)),
            StatusCode::CREATED => {
                let token: TokenData = serde_json::from_str(&resp.text().await?)?;
                Ok(Jwt(token.access))
            }
            StatusCode::SERVICE_UNAVAILABLE => Err(RichamsterError::ServiceUnavailable),
            StatusCode::BAD_REQUEST => {
                let text = resp.text().await?;
                let error: LoginResponseError = serde_json::from_str(&text)?;
                Err(RichamsterError::Login(error))
            }
            StatusCode::FORBIDDEN => {
                let text = resp.text().await?;
                let resp: LoginResponseError = serde_json::from_str(&text)?;
                Err(RichamsterError::InvalidCredential(resp))
            }
            status => Err(RichamsterError::UnsupportedResponseCode(
                status,
                resp.text().await?,
            )),
        }
    }

    pub async fn register_user(
        register_user: RegisterUser,
    ) -> Result<RegisterUserResponse, RichamsterError> {
        let RequestData(url, method) =
            Api::Authentication(AuthenticationApi::Register).request_data();
        let resp = Self::send_request(url, method, register_user).await?;

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
            Ok(OtpLoginResponse::Jwt(token.access))
        } else {
            let error: OtpLoginResponseError = serde_json::from_str(&resp.text().await?)?;
            Err(RichamsterError::Otp(error))
        }
    }

    pub async fn refresh_token(jwt_token: String) -> Result<TokenData, RichamsterError> {
        let RequestData(url, method) =
            Api::Authentication(AuthenticationApi::RefreshToken).request_data();
        let resp = Self::send_request(url, method, RefreshToken { refresh: jwt_token }).await?;
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
