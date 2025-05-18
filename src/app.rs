mod fetch_test_suite;
mod login;
mod test;

use crate::{
    cli::{Cli, Command},
    dto::config::Config,
};
use clap::Parser;

pub fn run(config: Config) {
    match Cli::parse().command {
        Command::Login { check } => login::run(&config, check),
        Command::FetchTestSuite { url } => fetch_test_suite::run(&config, url),
        Command::Test {
            language,
            task,
            test_cases,
            verbose,
        } => {
            test::run(&config, language, task, test_cases, verbose);
        }
    }
}
