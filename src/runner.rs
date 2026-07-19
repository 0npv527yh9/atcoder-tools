use crate::{
    cli::{Cli, Command, CookieCommand},
    dto::{config::Config, cookie::RevelSessionCookie, SessionData},
    infra::{
        atcoder::{self, Dao},
        config_loader::{self, ConfigLoadError},
        file_handler,
        http_handler::HttpHandler,
        terminal_handler,
    },
    usecase, RunOutcome,
};

pub(crate) fn run(cli: Cli) -> Result<RunOutcome, Error> {
    let config = config_loader::load_config()?;

    match cli.command {
        Command::Cookie { command } => {
            match command {
                CookieCommand::Import => {
                    let cookie_str =
                        terminal_handler::read_revel_session().map_err(Error::Terminal)?;
                    let revel_session_cookie = RevelSessionCookie::parse(&cookie_str)?;
                    let revel_session_cookie =
                        verify_and_save_session(&config, revel_session_cookie)?;

                    println!("Expires: {:?}", revel_session_cookie.expires_datetime());
                }
                CookieCommand::Check => {
                    let session_data: SessionData =
                        file_handler::load(&config.app_config.path.session_data)?;
                    let session_data =
                        verify_and_save_session(&config, session_data.revel_session_cookie)?;

                    println!("Expires: {:?}", session_data.expires_datetime());
                }
            }
            Ok(RunOutcome::Success)
        }
        Command::FetchTestSuite { url } => {
            let session_data: SessionData =
                file_handler::load(&config.app_config.path.session_data)?;
            let dao = session_data_to_dao(session_data.revel_session_cookie)?;
            usecase::fetch_test_suite::run(&config, &dao, url)?;
            save_dao(&config, dao)?;
            Ok(RunOutcome::Success)
        }
        Command::Test {
            language,
            task,
            test_cases,
            verbose,
        } => {
            if usecase::test::run(&config, language, task, test_cases, verbose)? {
                Ok(RunOutcome::Success)
            } else {
                Ok(RunOutcome::Failure)
            }
        }
        Command::Submit { .. } => Err(Error::Unimplemented("submit")),
    }
}

fn session_data_to_dao(revel_session_cookie: RevelSessionCookie) -> Result<Dao, Error> {
    let csrf_token = revel_session_cookie
        .csrf_token()
        .ok_or(Error::CsrfTokenNotFound)?;
    let cookie_store = revel_session_cookie.into_cookie_store()?;
    let http_handler =
        HttpHandler::with_cookie_store(cookie_store).map_err(atcoder::Error::from)?;
    Ok(Dao::new(http_handler, csrf_token))
}

fn verify_and_save_session(
    config: &Config,
    revel_session_cookie: RevelSessionCookie,
) -> Result<RevelSessionCookie, Error> {
    let dao = session_data_to_dao(revel_session_cookie)?;
    let dao = ensure_logged_in(config, dao)?;
    save_dao(config, dao)
}

fn ensure_logged_in(config: &Config, dao: Dao) -> Result<Dao, Error> {
    let logged_in = dao.check_login(&config.app_config.url.homepage)?;

    if !logged_in {
        return Err(Error::NotLoggedIn);
    }

    Ok(dao)
}

fn save_dao(config: &Config, dao: Dao) -> Result<RevelSessionCookie, Error> {
    let revel_session_cookie = dao.revel_session_cookie()?;
    file_handler::save(&config.app_config.path.session_data, &revel_session_cookie)?;
    Ok(revel_session_cookie)
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {
    #[error(transparent)]
    ConfigLoad(#[from] ConfigLoadError),

    #[error(transparent)]
    AtCoder(#[from] atcoder::Error),

    #[error(transparent)]
    File(#[from] file_handler::Error),

    #[error(transparent)]
    SessionCookie(#[from] crate::dto::cookie::Error),

    #[error("CSRF Token Not Found")]
    CsrfTokenNotFound,

    #[error("Cookie is not logged in")]
    NotLoggedIn,

    #[error("Terminal Input Error: {:?}", .0)]
    Terminal(#[source] std::io::Error),

    #[error(transparent)]
    FetchTestSuite(#[from] usecase::fetch_test_suite::Error),

    #[error(transparent)]
    Test(#[from] usecase::test::Error),

    #[error("{0} command is not implemented yet")]
    Unimplemented(&'static str),
}
