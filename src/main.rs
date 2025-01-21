mod cli;
mod config;
mod dao;
mod domain;
mod dto;
mod error;
mod handler;
mod service;
mod utils;

fn main() {
    let config = config::load_config("config.toml");
    cli::run(config);
}
