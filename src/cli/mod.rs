mod fetch_test_suite;
mod login;

use crate::config::Config;
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Login,

    /// Fetch test suite
    #[command(name = "fetch-test", visible_alias = "f")]
    FetchTestSuite {
        /// URL of a contest page or a task page
        url: String,
    },
}

pub fn run(config: Config) {
    match Cli::parse().command {
        Command::Login => login::login(&config.url.login, &config.file.session_data),
        Command::FetchTestSuite { url } => fetch_test_suite::fetch(&url, &config),
    }
}
