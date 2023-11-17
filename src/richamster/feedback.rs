use crate::api::FeedbackApi;
use crate::api::RequestPath;
use crate::api::{Api, RequestData};
use crate::errors::RichamsterError;
use crate::models::feedback::{ContactUs, ContactUsError, Messenger};
use reqwest::{Client, StatusCode};

pub struct Feedback;

impl Feedback {
    pub async fn messengers_list() -> Result<Vec<Messenger>, RichamsterError> {
        let RequestData(url, method) = Api::Feedback(FeedbackApi::Messengers).request_data();
        let resp = Client::new().request(method, url).send().await?;

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
        let RequestData(url, method) = Api::Feedback(FeedbackApi::ContactUs).request_data();
        let resp = Client::new()
            .request(method, url)
            .json(&contact_us)
            .send()
            .await?;

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
