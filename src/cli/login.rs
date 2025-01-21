use crate::{
    config::Config, dao::Dao, error::UnwrapOrExit, handler::http_handler::HttpHandler,
    service::login::LoginService,
};
use ureq::Agent;

pub fn login(config: &Config) {
    let http_handler = HttpHandler::new(Agent::new());
    let csrf_token = Dao::fetch_csrf_token(&http_handler, &config.url.homepage).unwrap_or_exit();
    let dao = Dao::new(http_handler, csrf_token);
    let login_service = LoginService::new(dao);

    login_service.login(&config.url.login).unwrap_or_exit();

    println!("Login Successful");

    let session_data_file = &config.file.session_data;
    login_service
        .save_session_data(session_data_file)
        .unwrap_or_exit();

    println!("{session_data_file} Created");
}
