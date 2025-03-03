mod fetch_test_suite;
mod login;

use crate::{domain::url::FetchTaskUrl, dto::config::Config};
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Login
    Login,

    /// Fetch test suite
    #[command(name = "fetch-test", visible_alias = "f")]
    FetchTestSuite {
        /// URL of a contest page or a task page
        ///
        /// The following formats are supported:
        /// - Contest Page URL: https://atcoder.jp/contests/<contest>
        /// - Task Page URL: https://atcoder.jp/contests/<contest>/task/<task>
        #[arg(verbatim_doc_comment)]
        url: FetchTaskUrl,
    },
}

pub fn run(config: Config) {
    match Cli::parse().command {
        Command::Login => login::login(&config),
        Command::FetchTestSuite { url } => fetch_test_suite::fetch(&config, url),
    }
}
