mod cli;
mod dto;
mod error;
mod infra;
mod runner;
mod usecase;
mod utils;

pub fn run() {
    runner::run();
}
