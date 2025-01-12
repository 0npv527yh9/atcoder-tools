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
}

pub fn run(config: Config) {
    match Cli::parse().command {
        Command::Login => login::login(&config.url.login, &config.file.session_data),
    }
}
