use crate::api::FeedbackApi;
use crate::api::RequestPath;
use crate::api::{Api, RequestData};
use crate::errors::RichamsterError;
use crate::models::feedback::Messenger;
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
}
