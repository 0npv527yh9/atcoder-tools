use crate::{
    cli::{Cli, Command},
    dto::{config::Config, SessionData},
    error::UnwrapOrExit,
    infra::{
        atcoder::{self, Dao},
        file_handler,
        http_handler::HttpHandler,
    },
    usecase,
};
use clap::Parser;
use ureq::Agent;

pub(crate) fn run() {
    let config = file_handler::load_config().unwrap_or_exit();

    match Cli::parse().command {
        Command::Login { check } => {
            if check {
                let dao = setup_dao_with_loading(&config).unwrap_or_exit();
                let (dao, logged_in) = usecase::login::check_login(&config, dao).unwrap_or_exit();

                if logged_in {
                    let session_data = save_dao(&config, dao).unwrap_or_exit();
                    println!("Expires: {:?}", session_data.expired_datetime());
                }
            } else {
                let dao = setup_dao_with_fetching(&config).unwrap_or_exit();
                let dao = usecase::login::login(&config, dao).unwrap_or_exit();
                save_dao(&config, dao).unwrap_or_exit();

                let session_data_file = &config.app_config.path.session_data;
                println!("{} Created", session_data_file.display());
            }
        }
        Command::FetchTestSuite { url } => {
            let dao = setup_dao_with_loading(&config).unwrap_or_exit();
            usecase::fetch_test_suite::run(&config, &dao, url).unwrap_or_exit();
            save_dao(&config, dao).unwrap_or_exit();
        }
        Command::Test {
            language,
            task,
            test_cases,
            verbose,
        } => {
            usecase::test::run(&config, language, task, test_cases, verbose);
        }
        Command::Submit { .. } => todo!(),
    }
}

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

fn save_dao(config: &Config, dao: Dao) -> Result<SessionData, file_handler::Error> {
    let session_data = dao.into_session_data();
    file_handler::save(&config.app_config.path.session_data, &session_data)?;
    Ok(session_data)
}
