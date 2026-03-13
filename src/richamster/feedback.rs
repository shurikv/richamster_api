use crate::api::{Api, FeedbackApi, RequestData, RequestPath};
use crate::errors::RichamsterError;
use crate::models::feedback::Messenger;
use crate::richamster::common::send_request;
use reqwest::StatusCode;

pub struct Feedback;

impl Feedback {
    pub async fn messengers_list() -> Result<Vec<Messenger>, RichamsterError> {
        let RequestData(url, method) = Api::Feedback(FeedbackApi::Messengers).request_data();
        let resp = send_request(url, method).await?;

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
}
