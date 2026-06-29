use crate::{
    dto::{
        config::Config,
        cookie::{extract_csrf_token, parse_request_cookie_header},
        SessionData,
    },
    infra::{atcoder, terminal_handler},
};

pub fn import(config: &Config) -> Result<SessionData, Error> {
    let cookie_header = terminal_handler::read_cookie_header().map_err(Error::Terminal)?;
    let cookies =
        parse_request_cookie_header(&cookie_header, config.app_config.url.homepage.as_str())?;
    let csrf_token = extract_csrf_token(&cookies).ok_or(Error::CsrfTokenNotFound)?;

    Ok(SessionData {
        cookies,
        csrf_token,
    })
}

pub fn check(config: &Config, dao: &atcoder::Dao) -> Result<bool, Error> {
    let logged_in = dao.check_login(&config.app_config.url.homepage)?;

    if logged_in {
        println!("Logged in");
    } else {
        println!("Not logged in");
    }

    Ok(logged_in)
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Cookie(#[from] crate::dto::cookie::Error),

    #[error("CSRF Token Not Found")]
    CsrfTokenNotFound,

    #[error(transparent)]
    Dao(#[from] atcoder::Error),

    #[error("Terminal Input Error: {:?}", .0)]
    Terminal(#[source] std::io::Error),
}
