use crate::domain::{page_type, url};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub file: File,
    pub url: Url,
}

#[derive(Serialize, Deserialize)]
pub struct File {
    pub session_data: String,
    pub tasks_info: String,
    pub test: String,
}

#[derive(Serialize, Deserialize)]
pub struct Url {
    pub homepage: url::Url<page_type::Home>,
    pub login: url::Url<page_type::Login>,
}
