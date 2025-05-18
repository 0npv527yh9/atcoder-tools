mod fetch_test_suite;
mod login;
mod test;

use crate::{
    cli::{Cli, Command},
    dao::{self, Dao},
    dto::{config::Config, SessionData},
    handler::{file_handler, http_handler::HttpHandler},
};
use clap::Parser;
use ureq::Agent;

pub fn run(config: Config) {
    match Cli::parse().command {
        Command::Login { check } => login::run(&config, check),
        Command::FetchTestSuite { url } => fetch_test_suite::run(&config, url),
        Command::Test {
            language,
            task,
            test_cases,
            verbose,
        } => {
            test::run(&config, language, task, test_cases, verbose);
        }
    }
}

fn setup_dao_with_fetching(config: &Config) -> Result<Dao, dao::Error> {
    let http_handler = HttpHandler::new(Agent::new());
    let csrf_token = Dao::fetch_csrf_token(&http_handler, &config.app_config.url.homepage)?;
    Ok(Dao::new(http_handler, csrf_token))
}

fn setup_dao_with_loading(config: &Config) -> Result<Dao, file_handler::Error> {
    let SessionData {
        cookies,
        csrf_token,
    } = file_handler::load(&config.app_config.path.session_data)?;

    let http_handler = HttpHandler::with_cookies(cookies);
    Ok(Dao::new(http_handler, csrf_token))
}

fn save_dao(config: &Config, dao: Dao) -> Result<(), file_handler::Error> {
    let session_data = dao.into_session_data();
    file_handler::save(&config.app_config.path.session_data, &session_data)
}
