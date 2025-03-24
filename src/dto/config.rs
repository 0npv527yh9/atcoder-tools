use crate::domain::{page_type, url};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub file: File,
    pub url: Url,
}

#[derive(Serialize, Deserialize)]
pub struct File {
    pub session_data: PathBuf,
    pub tasks_info: PathBuf,
    pub test: PathBuf,
}

#[derive(Serialize, Deserialize)]
pub struct Url {
    pub homepage: url::Url<page_type::Home>,
    pub login: url::Url<page_type::Login>,
}
