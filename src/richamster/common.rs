use hmac_sha256::HMAC;
use reqwest::RequestBuilder;
use secrecy::{ExposeSecret, Secret};

const HEADER_API_KEY: &str = "Api-Key";
const HEADER_SIGNATURE: &str = "Signature";
const HEADER_AUTH: &str = "Authorization";
const JWT: &str = "JWT";

#[derive(Debug, Clone)]
pub struct JwtToken(pub Secret<String>);

impl JwtToken {
    fn value(&self) -> &str {
        self.0.expose_secret()
    }
}

#[derive(Debug, Clone)]
pub struct ApiKey(pub Secret<String>);

impl ApiKey {
    fn value(&self) -> &str {
        self.0.expose_secret()
    }
}

#[derive(Debug, Clone)]
pub struct SecretKey(pub Secret<String>);

impl SecretKey {
    fn value(&self) -> &str {
        self.0.expose_secret()
    }
}

#[derive(Clone, Debug)]
pub enum AuthState {
    Unauthorized,
    JwtTokenAuth(JwtToken),
    ApiSecretKeyAuth(ApiKey, SecretKey),
    JwtTokenWithApiSecretKeyAuth(JwtToken, ApiKey, SecretKey),
}

pub trait HeaderCompose {
    fn compose(self, auth_state: &AuthState) -> RequestBuilder;
    fn compose_with_payload(self, auth_state: &AuthState, payload: &str) -> RequestBuilder;
}

impl HeaderCompose for RequestBuilder {
    fn compose(self, auth_state: &AuthState) -> RequestBuilder {
        match auth_state {
            AuthState::Unauthorized => self,
            AuthState::JwtTokenAuth(jwt_token) => {
                AuthState::insert_jwt_token_header(self, jwt_token)
            }
            AuthState::ApiSecretKeyAuth(api, secret) => {
                AuthState::insert_keys_headers(self, api, secret, "")
            }
            AuthState::JwtTokenWithApiSecretKeyAuth(jwt_token, api, secret) => {
                let builder = AuthState::insert_jwt_token_header(self, jwt_token);
                AuthState::insert_keys_headers(builder, api, secret, "")
            }
        }
    }

    fn compose_with_payload(self, auth_state: &AuthState, payload: &str) -> RequestBuilder {
        match auth_state {
            AuthState::Unauthorized => self,
            AuthState::JwtTokenAuth(jwt_token) => {
                AuthState::insert_jwt_token_header(self, jwt_token)
            }
            AuthState::ApiSecretKeyAuth(api, secret) => {
                AuthState::insert_keys_headers(self, api, secret, payload)
            }
            AuthState::JwtTokenWithApiSecretKeyAuth(jwt_token, api, secret) => {
                let builder = AuthState::insert_jwt_token_header(self, jwt_token);
                AuthState::insert_keys_headers(builder, api, secret, payload)
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
