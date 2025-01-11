use crate::{dao::Dao, handler::http_handler::HttpHandler, service::login::LoginService};
use ureq::Agent;

pub fn login(url: &str, session_data_file: &str) {
    let http_handler = HttpHandler::new(Agent::new());
    let dao = Dao::with_fetching(http_handler, url).unwrap();
    let login_service = LoginService::new(dao);

    login_service.login(url).unwrap();
    login_service.save(session_data_file).unwrap();
}
