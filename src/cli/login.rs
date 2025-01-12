use crate::{
    dao::Dao, error::UnwrapOrExit, handler::http_handler::HttpHandler, service::login::LoginService,
};
use ureq::Agent;

pub fn login(url: &str, session_data_file: &str) {
    let http_handler = HttpHandler::new(Agent::new());
    let csrf_token = Dao::fetch_csrf_token(&http_handler, url).unwrap_or_exit();
    let dao = Dao::new(http_handler, csrf_token);
    let login_service = LoginService::new(dao);

    login_service.login(url).unwrap_or_exit();

    println!("Login Successful");

    login_service
        .save_session_data(session_data_file)
        .unwrap_or_exit();

    println!("{session_data_file} Created");
}
