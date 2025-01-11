mod cli;
mod dao;
mod dto;
mod handler;
mod parser;
mod service;
mod utils;

fn main() {
    let login_url = "https://atcoder.jp/login";
    let session_data_file = "session_data.json";
    cli::run(login_url, session_data_file);
}
