use crate::{
    cli::{Cli, Command, CookieCommand},
    dto::{config::Config, SessionData},
    infra::{
        atcoder::{self, Dao},
        config_loader::{self, ConfigLoadError},
        file_handler,
        http_handler::HttpHandler,
    },
    usecase, RunOutcome,
};

pub(crate) fn run(cli: Cli) -> Result<RunOutcome, Error> {
    let config = config_loader::load_config()?;

    match cli.command {
        Command::Cookie { command } => {
            match command {
                CookieCommand::Import => {
                    let session_data = usecase::cookie::import(&config)?;
                    save_session_data(&config, &session_data)?;

                    let session_data_file = &config.app_config.path.session_data;
                    println!("{} Created", session_data_file.display());
                    println!("Run `cookie check` to verify the saved cookies.");
                }
                CookieCommand::Check => {
                    let session_data = load_session_data(&config)?;
                    let dao = session_data_to_dao(session_data);
                    let logged_in = usecase::cookie::check(&config, &dao)?;

                    if logged_in {
                        let session_data = save_dao(&config, dao)?;
                        println!("Expires: {:?}", session_data.expired_datetime());
                    }
                }
            }
            Ok(RunOutcome::Success)
        }
        Command::FetchTestSuite { url } => {
            let session_data = load_session_data(&config)?;
            let dao = session_data_to_dao(session_data);
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

fn load_session_data(config: &Config) -> Result<SessionData, file_handler::Error> {
    file_handler::load(&config.app_config.path.session_data)
}

fn session_data_to_dao(session_data: SessionData) -> Dao {
    let csrf_token = session_data.csrf_token;
    let http_handler = HttpHandler::with_cookies(session_data.cookies);
    Dao::new(http_handler, csrf_token)
}

fn save_dao(config: &Config, dao: Dao) -> Result<SessionData, file_handler::Error> {
    let session_data = dao.into_session_data();
    save_session_data(config, &session_data)?;
    Ok(session_data)
}

fn save_session_data(
    config: &Config,
    session_data: &SessionData,
) -> Result<(), file_handler::Error> {
    file_handler::save(&config.app_config.path.session_data, session_data)
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
    Cookie(#[from] usecase::cookie::Error),

    #[error(transparent)]
    FetchTestSuite(#[from] usecase::fetch_test_suite::Error),

    #[error(transparent)]
    Test(#[from] usecase::test::Error),

    #[error("{0} command is not implemented yet")]
    Unimplemented(&'static str),
}
