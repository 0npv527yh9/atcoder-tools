use super::{setup_dao_with_fetching, setup_dao_with_loading};
use crate::{
    app::save_dao,
    dao::{self, Dao},
    domain::{page_type, url::Url},
    dto::config::Config,
    error::UnwrapOrExit,
    handler::{file_handler, terminal_handler},
};

pub fn run(config: &Config, check: bool) {
    if check {
        let dao = setup_dao_with_loading(config).unwrap_or_exit();
        check_login(config, dao).unwrap_or_exit();
    } else {
        let dao = setup_dao_with_fetching(config).unwrap_or_exit();
        login(config, dao).unwrap_or_exit();
    }
}

fn login(config: &Config, dao: Dao) -> Result<(), Error> {
    interactive_login(&dao, &config.app_config.url.login)?;

    println!("Login Successful");

    save_dao(config, dao)?;
    let session_data_file = &config.app_config.path.session_data;
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

fn check_login(config: &Config, dao: Dao) -> Result<bool, Error> {
    let logged_in = dao.check_login(&config.app_config.url.homepage)?;

    if logged_in {
        println!("Logged in");

        let session_data = dao.into_session_data();
        file_handler::save(&config.app_config.path.session_data, &session_data);
        println!("Expires: {:?}", session_data.expired_datetime());
    } else {
        println!("Not logged in");
    }

    Ok(logged_in)
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
