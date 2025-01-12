use crate::{dao::Dao, handler::http_handler::HttpHandler, service::login::LoginService};
use ureq::Agent;

pub fn login(url: &str, session_data_file: &str) -> Result<(), ()> {
    let http_handler = HttpHandler::new(Agent::new());
    let csrf_token =
        Dao::fetch_csrf_token(&http_handler, url).map_err(|error| println!("{error}"))?;
    let dao = Dao::new(http_handler, csrf_token);
    let login_service = LoginService::new(dao);

    login_service
        .login(url)
        .map_err(|error| println!("{error}"))?;

    login_service.save(session_data_file).unwrap();
    Ok(())
}
