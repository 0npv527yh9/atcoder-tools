mod cli;
mod dto;
mod error;
mod infra;
mod usecase;
mod utils;

use clap::Parser;
use cli::{Cli, Command};
use error::UnwrapOrExit;
use infra::file_handler;

fn main() {
    let config = file_handler::load_config().unwrap_or_exit();
    match Cli::parse().command {
        Command::Login { check } => usecase::login::run(&config, check),
        Command::FetchTestSuite { url } => usecase::fetch_test_suite::run(&config, url),
        Command::Test {
            language,
            task,
            test_cases,
            verbose,
        } => {
            usecase::test::run(&config, language, task, test_cases, verbose);
        }
        Command::Submit { language, task } => todo!(),
    }
}
