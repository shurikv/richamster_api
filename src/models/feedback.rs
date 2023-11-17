use serde_derive::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use url::Url;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct ContactUs {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub question: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Messenger {
    pub id: i32,
    pub order: i32,
    pub title: String,
    pub icon: Url,
    pub link: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct ContactUsError {
    pub name: Option<Vec<String>>,
    pub email: Option<Vec<String>>,
    pub phone: Option<Vec<String>>,
    pub question: Option<Vec<String>>,
    pub non_field_errors: Option<Vec<String>>,
}

impl Display for ContactUsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
