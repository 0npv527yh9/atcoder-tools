use crate::{
    config::Config,
    dao::Dao,
    dto::SessionData,
    error::UnwrapOrExit,
    handler::{file_handler, http_handler::HttpHandler},
    service::fetch_test_case::FetchTestSuiteService,
};

pub fn fetch(url: &str, config: &Config) {
    let SessionData {
        cookies,
        csrf_token,
    } = file_handler::load_session_data(&config.file.session_data).unwrap_or_exit();

    let http_handler = HttpHandler::with_cookies(cookies);
    let dao = Dao::new(http_handler, csrf_token);
    let fetch_service = FetchTestSuiteService::new(dao);

    let tasks = fetch_service.fetch_test_suite(url, config).unwrap_or_exit();

    println!("Saved: {tasks:?}");
}
