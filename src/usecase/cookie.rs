use crate::{
    dto::{config::Config, cookie::RevelSessionCookie},
    infra::{atcoder, terminal_handler},
};

pub fn import() -> Result<RevelSessionCookie, Error> {
    let cookie_str = terminal_handler::read_revel_session().map_err(Error::Terminal)?;
    let revel_session_cookie = RevelSessionCookie::parse(&cookie_str)?;
    Ok(revel_session_cookie)
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

    #[error("Imported cookies are not logged in")]
    ImportedCookieNotLoggedIn,

    #[error(transparent)]
    Dao(#[from] atcoder::Error),

    #[error("Terminal Input Error: {:?}", .0)]
    Terminal(#[source] std::io::Error),
}
