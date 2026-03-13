use hmac_sha256::HMAC;
use reqwest::{IntoUrl, Method, RequestBuilder, Response, StatusCode};
use secrecy::{ExposeSecret, SecretBox};
use serde::de::DeserializeOwned;

use crate::{api::CLIENT, errors::RichamsterError};

const HEADER_API_KEY: &str = "Api-Key";
const HEADER_SIGNATURE: &str = "Signature";
const HEADER_AUTH: &str = "Authorization";
const JWT: &str = "JWT";

#[derive(Debug)]
pub struct JwtToken(pub SecretBox<String>);

impl JwtToken {
    pub fn new(token: String) -> JwtToken {
        Self(SecretBox::new(Box::new(token)))
    }

    fn value(&self) -> &str {
        self.0.expose_secret()
    }
}

#[derive(Debug)]
pub struct ApiKey(pub SecretBox<String>);

impl ApiKey {
    pub fn new(api_key: String) -> ApiKey {
        Self(SecretBox::new(Box::new(api_key)))
    }

    fn value(&self) -> &str {
        self.0.expose_secret()
    }
}

#[derive(Debug)]
pub struct SecretKey(pub SecretBox<String>);

impl SecretKey {
    pub fn new(secret_key: String) -> SecretKey {
        Self(SecretBox::new(Box::new(secret_key)))
    }

    fn value(&self) -> &str {
        self.0.expose_secret()
    }
}

#[derive(Debug, Default)]
pub enum AuthState {
    #[default]
    Unauthorized,
    JwtTokenAuth(JwtToken),
    ApiSecretKeyAuth(ApiKey, SecretKey),
    JwtTokenWithApiSecretKeyAuth(JwtToken, ApiKey, SecretKey),
}

pub trait HeaderCompose {
    fn compose(self, auth_state: &AuthState, payload: Option<&str>) -> RequestBuilder;
}

impl HeaderCompose for RequestBuilder {
    fn compose(self, auth_state: &AuthState, payload: Option<&str>) -> RequestBuilder {
        match auth_state {
            AuthState::Unauthorized => self,
            AuthState::JwtTokenAuth(jwt_token) => {
                AuthState::insert_jwt_token_header(self, jwt_token)
            }
            AuthState::ApiSecretKeyAuth(api, secret) => {
                AuthState::insert_keys_headers(self, api, secret, payload.unwrap_or(""))
            }
            AuthState::JwtTokenWithApiSecretKeyAuth(jwt_token, api, secret) => {
                let builder = AuthState::insert_jwt_token_header(self, jwt_token);
                AuthState::insert_keys_headers(builder, api, secret, payload.unwrap_or(""))
            }
        }
    }
}

impl AuthState {
    fn insert_keys_headers(
        builder: RequestBuilder,
        api: &ApiKey,
        secret: &SecretKey,
        payload: &str,
    ) -> RequestBuilder {
        let hmac = HMAC::mac(payload, secret.value());
        let hex = hex::encode(hmac);
        builder
            .header(HEADER_API_KEY, api.value())
            .header(HEADER_SIGNATURE, hex)
    }

    fn insert_jwt_token_header(builder: RequestBuilder, jwt_token: &JwtToken) -> RequestBuilder {
        builder.header(HEADER_AUTH, format!("{} {}", JWT, jwt_token.value()))
    }
}

pub async fn process_response<T: DeserializeOwned>(
    response: Response,
) -> Result<T, RichamsterError> {
    match response.status() {
        StatusCode::OK => {
            let res = response.text().await?;
            Ok(serde_json::from_str(res.as_str())?)
        }
        StatusCode::UNAUTHORIZED => Err(RichamsterError::UnauthorizedAccess),
        status => Err(RichamsterError::UnsupportedResponseCode(
            status,
            response.text().await?,
        )),
    }
}

pub async fn send_request(url: impl IntoUrl, method: Method) -> Result<Response, RichamsterError> {
    Ok(CLIENT.request(method, url).send().await?)
}

pub async fn send_request_with_auth(
    url: impl IntoUrl,
    method: Method,
    auth_state: &AuthState,
) -> Result<Response, RichamsterError> {
    Ok(CLIENT
        .request(method, url)
        .compose(auth_state, None)
        .send()
        .await?)
}

pub async fn send_request_with_body(
    url: impl IntoUrl,
    method: Method,
    body: String,
) -> Result<Response, RichamsterError> {
    Ok(CLIENT
        .request(method, url)
        .body(body)
        .header("Content-Type", "application/json")
        .send()
        .await?)
}

pub async fn send_request_with_body_and_auth(
    url: impl IntoUrl,
    method: Method,
    body: String,
    auth_state: &AuthState,
) -> Result<Response, RichamsterError> {
    Ok(CLIENT
        .request(method, url)
        .compose(auth_state, Some(body.as_str()))
        .body(body)
        .header("Content-Type", "application/json")
        .send()
        .await?)
}
