mod cli;
mod dto;
mod infra;
mod runner;
mod usecase;
mod utils;

use clap::Parser;

pub struct ParsedCli(cli::Cli);

pub enum RunOutcome {
    Success,
    Failure,
}

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub struct RunError(#[from] runner::Error);

pub fn parse_cli() -> ParsedCli {
    ParsedCli(cli::Cli::parse())
}

pub fn run(cli: ParsedCli) -> Result<RunOutcome, RunError> {
    runner::run(cli.0).map_err(Into::into)
}
