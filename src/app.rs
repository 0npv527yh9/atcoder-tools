mod fetch_test_suite;
mod login;

use crate::{
    cli::{Cli, Command},
    dto::config::Config,
};
use clap::Parser;

pub fn run(config: Config) {
    match Cli::parse().command {
        Command::Login => login::run(&config),
        Command::FetchTestSuite { url } => fetch_test_suite::run(&config, url),
    }
}
