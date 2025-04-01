mod app;
mod cli;
mod dao;
mod domain;
mod dto;
mod error;
mod handler;
mod utils;

use dto::config::Config;

fn main() {
    let config = load_config();
    app::run(config);
}

fn load_config() -> Config {
    let config = include_str!("../config.toml");
    toml::from_str(config).expect("config.toml is not valid")
}
