use crate::api::Api;
use crate::api::FeedbackApi;
use crate::api::RequestPath;
use crate::errors::RichamsterError;
use crate::models::feedback::{ContactUs, ContactUsError, Messenger};
use crate::send_request;
use reqwest::StatusCode;

pub struct Feedback;

impl Feedback {
    pub async fn messengers_list() -> Result<Vec<Messenger>, RichamsterError> {
        let resp = send_request!(Api::Feedback(FeedbackApi::Messengers).full_url(), get);

        match resp.status() {
            StatusCode::OK => {
                let messengers: Vec<Messenger> = serde_json::from_str(&resp.text().await?)?;
                Ok(messengers)
            }
            status => Err(RichamsterError::UnsupportedResponseCode(
                status,
                resp.text().await?,
            )),
        }
    }

    pub async fn contact_us(contact_us: ContactUs) -> Result<ContactUs, RichamsterError> {
        let payload = serde_json::to_string(&contact_us)?;
        let resp = send_request!(
            Api::Feedback(FeedbackApi::ContactUs).full_url(),
            payload,
            post
        );

        match resp.status() {
            StatusCode::CREATED => {
                let contact_us: ContactUs = serde_json::from_str(&resp.text().await?)?;
                Ok(contact_us)
            }
            StatusCode::BAD_REQUEST => {
                let error: ContactUsError = serde_json::from_str(&resp.text().await?)?;
                Err(RichamsterError::ContactUs(error))
            }
            status => Err(RichamsterError::UnsupportedResponseCode(
                status,
                resp.text().await?,
            )),
        }
    }
}
