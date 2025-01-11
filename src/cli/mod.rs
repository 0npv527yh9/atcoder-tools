mod login;

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

pub fn run(login_url: &str, session_data_file: &str) {
    match Cli::parse().command {
        Command::Login => login::login(login_url, session_data_file),
    }
}
