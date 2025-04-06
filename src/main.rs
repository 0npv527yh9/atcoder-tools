mod app;
mod cli;
mod dao;
mod domain;
mod dto;
mod error;
mod handler;
mod utils;

use error::UnwrapOrExit;
use handler::file_handler;

fn main() {
    let config = file_handler::load_config().unwrap_or_exit();
    app::run(config);
}
