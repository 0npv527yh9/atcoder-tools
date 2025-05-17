use serde::Deserialize;

use crate::domain::{page_type, url};
use crate::dto::Command;
use std::path::PathBuf;
pub struct Config {
    pub app_config: AppConfig,
    pub user_config: UserConfig,
}

#[derive(Deserialize)]
pub struct AppConfig {
    pub path: Path,
    pub url: Url,
}

#[derive(Deserialize)]
pub struct Path {
    pub session_data: PathBuf,
    pub tasks_info: PathBuf,
    pub test: PathBuf,
    pub user_config: PathBuf,
    pub metadata: PathBuf,
}

#[derive(Deserialize)]
pub struct Url {
    pub homepage: url::Url<page_type::Home>,
    pub login: url::Url<page_type::Login>,
}

#[derive(Deserialize)]
pub struct UserConfig {
    language: Vec<LanguageConfig>,
}

impl UserConfig {
    pub fn language_config(&self, language: &str) -> Option<&LanguageConfig> {
        self.language.iter().find(|config| config.name == language)
    }
}

#[derive(Deserialize)]
pub struct LanguageConfig {
    name: String,
    id: String,
    src_path: PathBuf,
    pub compile: Option<Command>,
    pub execute: Command,
}
