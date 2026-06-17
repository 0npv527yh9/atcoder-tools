pub mod fetch_test_suite;
pub mod login;
pub mod test;

use crate::{
    dto::{config::Config, SessionData},
    infra::{
        atcoder::{self, Dao},
        file_handler,
        http_handler::HttpHandler,
    },
};
use ureq::Agent;

fn setup_dao_with_fetching(config: &Config) -> Result<Dao, atcoder::Error> {
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
