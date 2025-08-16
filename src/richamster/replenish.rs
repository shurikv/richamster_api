use crate::api::token::Token;
use crate::api::{Api, ReplenishApi, RequestData, RequestPath};
use crate::errors::RichamsterError;
use crate::models::replenish::{P2PReplenish, ReplenishInfo};
use crate::richamster::common;
use crate::richamster::common::{AuthState, HeaderCompose};
use crate::send_request;
use reqwest::StatusCode;
use secrecy::SecretBox;
use crate::models::common::CurrencyChannel;

pub struct Replenish {
    auth_state: AuthState,
}

impl Replenish {
    pub fn with_jwt_token(token: String) -> Self {
        Self {
            auth_state: AuthState::JwtTokenAuth(common::JwtToken(SecretBox::new(Box::new(token)))),
        }
    }

    pub fn with_keys(api_key: String, secret_key: String) -> Self {
        Self {
            auth_state: AuthState::ApiSecretKeyAuth(
                common::ApiKey(SecretBox::new(Box::new(api_key))),
                common::SecretKey(SecretBox::new(Box::new(secret_key))),
            ),
        }
    }
}

impl Replenish {
    pub async fn replenish_info(&self, currency_name: Token, currency_channel: String) -> Result<ReplenishInfo, RichamsterError> {
        let RequestData(url, method) = Api::Replenish(ReplenishApi::ReplenishInfo).request_data();
        let path = format!("{}/{}/", currency_name.as_ref(), currency_channel);
        let url = url.join(&path)?;
        let resp = send_request!(url, method, self.auth_state);
        match resp.status() {
            StatusCode::OK => {
                let string = resp.text().await?;
                let balance: ReplenishInfo = serde_json::from_str(&string)?;
                Ok(balance)
            }
            StatusCode::UNAUTHORIZED => Err(RichamsterError::UnauthorizedAccess),
            StatusCode::NOT_FOUND => Err(RichamsterError::ReplenishInfoNotFound(
                currency_name, currency_channel,
            )),
            status => Err(RichamsterError::UnsupportedResponseCode(
                status,
                resp.text().await?,
            )),
        }
    }

    pub async fn replenish_channels_info(&self, currency_name: Token) -> Result<Vec<CurrencyChannel>, RichamsterError> {
        let RequestData(mut url, method) = Api::Replenish(ReplenishApi::ReplenishChannelsInfo).request_data();
        url = url.join(currency_name.as_ref())?;
        let resp = send_request!(url, method, self.auth_state);
        match resp.status() {
            StatusCode::OK => {
                let string = resp.text().await?;
                let channels: Vec<CurrencyChannel> = serde_json::from_str(&string)?;
                Ok(channels)
            }
            StatusCode::UNAUTHORIZED => Err(RichamsterError::UnauthorizedAccess),
            status => Err(RichamsterError::UnsupportedResponseCode(
                status,
                resp.text().await?,
            )),
        }
    }

    pub async fn replenish_p2p(&self, replenish: P2PReplenish) -> Result<P2PReplenish, RichamsterError> {
        let RequestData(url, method) = Api::Replenish(ReplenishApi::P2PReplenish).request_data();
        let resp = send_request!(url, method, self.auth_state, serde_json::to_string(&replenish)?);
        match resp.status() {
            StatusCode::CREATED => {
                let string = resp.text().await?;
                let p2p_replenish: P2PReplenish = serde_json::from_str(&string)?;
                Ok(p2p_replenish)
            }
            StatusCode::UNAUTHORIZED => Err(RichamsterError::UnauthorizedAccess),
            status => Err(RichamsterError::UnsupportedResponseCode(
                status,
                resp.text().await?,
            )),
        }
    }
}
