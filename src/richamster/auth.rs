use crate::api::AuthenticationApi::TwoFactorLogin;
use crate::api::{Api, AuthenticationApi, RequestPath};
use crate::errors::RichamsterError;
use crate::models::authentication::LoginResponse::{Jwt, RequiresTwoFactor};
use crate::models::authentication::{
    Login, LoginResponse, LoginResponseError, NonFieldsError, OtpLogin, OtpLoginResponse,
    OtpLoginResponseError, RegisterUser, RegisterUserError, RegisterUserResponse, TokenData,
};
use crate::send_request;
use reqwest::StatusCode;

pub struct Auth;

impl Auth {
    pub async fn login(
        username: impl AsRef<str>,
        password: impl AsRef<str>,
    ) -> Result<LoginResponse, RichamsterError> {
        let login = Login::new(username.as_ref(), password.as_ref());
        let url = Api::Authentication(AuthenticationApi::Login).full_url();
        let payload = serde_json::to_string(&login)?;
        let resp = send_request!(url, payload, post);
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
        let payload = serde_json::to_string(&user_creation)?;
        let resp = send_request!(
            Api::Authentication(AuthenticationApi::Register).full_url(),
            payload,
            post
        );

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
        let payload = serde_json::to_string(&OtpLogin { otp_token })?;
        let resp = send_request!(
            Api::Authentication(TwoFactorLogin).full_url(),
            payload,
            post
        );

        if resp.status() == StatusCode::CREATED {
            let token: TokenData = serde_json::from_str(&resp.text().await?)?;
            Ok(OtpLoginResponse::Jwt(token.jwt_token))
        } else {
            let error: OtpLoginResponseError = serde_json::from_str(&resp.text().await?)?;
            Err(RichamsterError::Otp(error))
        }
    }

    pub async fn verify_token(jwt_token: String) -> Result<TokenData, RichamsterError> {
        let payload = serde_json::to_string(&TokenData { jwt_token })?;
        let resp = send_request!(
            Api::Authentication(AuthenticationApi::VerifyToken).full_url(),
            payload,
            post
        );

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
        let payload = serde_json::to_string(&TokenData { jwt_token })?;
        let resp = send_request!(
            Api::Authentication(AuthenticationApi::RefreshToken).full_url(),
            payload,
            post
        );

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
