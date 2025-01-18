mod fetch_test_suites;
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

    /// Fetch test suites
    #[command(name = "fetch-test", visible_alias = "f")]
    FetchTestSuites {
        /// URL of a contest page or a task page
        url: String,
    },
}

pub fn run(config: Config) {
    match Cli::parse().command {
        Command::Login => login::login(&config.url.login, &config.file.session_data),
        Command::FetchTestSuites { url } => fetch_test_suites::fetch(&url, &config),
    }
}
