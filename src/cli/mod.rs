mod login;

use crate::service::{self, login::LoginService};
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

#[derive(Debug)]
pub enum Error {
    LoginError(service::login::Error),
}

pub fn run(login_url: &str) -> Result<(), Error> {
    match Cli::parse().command {
        Command::Login => {
            let login_service =
                LoginService::with_fetching(login_url).map_err(Error::LoginError)?;
            login::login(login_service).map_err(Error::LoginError)
        }
    }
}
