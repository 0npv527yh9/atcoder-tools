use crate::error::ExpectOrExit;
use serde::{Deserialize, Serialize};
use std::fs;

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
    pub homepage: String,
    pub login: String,
}

pub fn load_config(file_path: &str) -> Config {
    let config = fs::read_to_string(file_path).expect_or_exit(&format!("Not Found: {file_path}"));
    toml::from_str(&config).expect_or_exit(&format!("Invalid TOML: {file_path}"))
}
