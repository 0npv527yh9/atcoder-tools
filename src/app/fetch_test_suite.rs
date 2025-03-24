use crate::{
    dao::Dao,
    domain::url::FetchTaskUrl,
    dto::{config::Config, SessionData},
    error::UnwrapOrExit,
    handler::{file_handler, http_handler::HttpHandler},
    service::fetch_test_case::FetchTestSuiteService,
};

pub fn fetch(config: &Config, task_url: FetchTaskUrl) {
    let SessionData {
        cookies,
        csrf_token,
    } = file_handler::load(&config.file.session_data).unwrap_or_exit();

    let http_handler = HttpHandler::with_cookies(cookies);
    let dao = Dao::new(http_handler, csrf_token);
    let fetch_service = FetchTestSuiteService::new(dao);

    let tasks = fetch_service
        .fetch_test_suite(config, task_url)
        .unwrap_or_exit();

    println!("Saved: {tasks:?}");
}
