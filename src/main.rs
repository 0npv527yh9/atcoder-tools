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

fn main() {
    let config = file_handler::load_toml("config.toml").unwrap_or_exit();
    cli::run(config);
}
