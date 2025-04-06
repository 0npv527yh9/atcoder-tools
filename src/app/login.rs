use crate::{
    dao::{self, Dao},
    domain::{page_type, url::Url},
    dto::config::Config,
    error::UnwrapOrExit,
    handler::{file_handler, http_handler::HttpHandler, terminal_handler},
};
use ureq::Agent;

pub fn run(config: &Config) {
    let dao = setup(config);
    login(config, dao).unwrap_or_exit();
}

fn setup(config: &Config) -> Dao {
    let http_handler = HttpHandler::new(Agent::new());
    let csrf_token =
        Dao::fetch_csrf_token(&http_handler, &config.app_config.url.homepage).unwrap_or_exit();
    Dao::new(http_handler, csrf_token)
}

fn login(config: &Config, dao: Dao) -> Result<(), Error> {
    interactive_login(&dao, &config.app_config.url.login)?;

    println!("Login Successful");

    let session_data_file = &config.app_config.path.session_data;
    file_handler::save(session_data_file, &dao.into_session_data())?;
    println!("{} Created", session_data_file.display());

    Ok(())
}

fn interactive_login(dao: &Dao, url: &Url<page_type::Login>) -> Result<(), Error> {
    let credentials = terminal_handler::read_credentials().map_err(Error::Terminal)?;
    dao.login(credentials, url).or_else(|error| {
        let should_retry = terminal_handler::ask_for_retry().map_err(Error::Terminal)?;
        if should_retry {
            interactive_login(dao, url)
        } else {
            Err(Error::Dao(error))
        }
    })
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Dao(#[from] dao::Error),

    #[error("Terminal Input Error: {:?}", .0)]
    Terminal(#[source] std::io::Error),

    #[error(transparent)]
    FileHandler(#[from] file_handler::Error),
}
