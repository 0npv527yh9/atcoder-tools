use crate::{
    dto::config::Config,
    infra::{
        atcoder::{self, page_type, url::Url, Dao},
        terminal_handler,
    },
};

pub fn login(config: &Config, dao: Dao) -> Result<Dao, Error> {
    interactive_login(&dao, &config.app_config.url.login)?;

    println!("Login Successful");

    Ok(dao)
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

pub fn check_login(config: &Config, dao: Dao) -> Result<(Dao, bool), Error> {
    let logged_in = dao.check_login(&config.app_config.url.homepage)?;

    if logged_in {
        println!("Logged in");
    } else {
        println!("Not logged in");
    }

    Ok((dao, logged_in))
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Dao(#[from] atcoder::Error),

    #[error("Terminal Input Error: {:?}", .0)]
    Terminal(#[source] std::io::Error),
}
