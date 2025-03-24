mod app;
mod cli;
mod dao;
mod domain;
mod dto;
mod error;
mod handler;
mod service;
mod utils;

use error::UnwrapOrExit;
use handler::file_handler;
use std::path::Path;

fn main() {
    let config = file_handler::load_toml(Path::new("config.toml")).unwrap_or_exit();
    app::run(config);
}
