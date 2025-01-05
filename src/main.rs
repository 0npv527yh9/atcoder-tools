mod cli;
mod dto;
mod handler;
mod parser;
mod service;
mod utils;

use cli::Error;

fn main() -> Result<(), Error> {
    let login_url = "https://atcoder.jp/login";
    cli::run(login_url)
}
