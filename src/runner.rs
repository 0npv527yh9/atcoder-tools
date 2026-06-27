use crate::{
    cli::{Cli, Command},
    dto::{config::Config, SessionData},
    infra::{
        atcoder::{self, Dao},
        config_loader::{self, ConfigLoadError},
        file_handler,
        http_handler::HttpHandler,
    },
    usecase, RunOutcome,
};
use ureq::Agent;

pub(crate) fn run(cli: Cli) -> Result<RunOutcome, Error> {
    let config = config_loader::load_config()?;

    match cli.command {
        Command::Login { check } => {
            if check {
                let dao = setup_dao_with_loading(&config)?;
                let (dao, logged_in) = usecase::login::check_login(&config, dao)?;

                if logged_in {
                    let session_data = save_dao(&config, dao)?;
                    println!("Expires: {:?}", session_data.expired_datetime());
                }
            } else {
                let dao = setup_dao_with_fetching(&config)?;
                let dao = usecase::login::login(&config, dao)?;
                save_dao(&config, dao)?;

                let session_data_file = &config.app_config.path.session_data;
                println!("{} Created", session_data_file.display());
            }
            Ok(RunOutcome::Success)
        }
        Command::FetchTestSuite { url } => {
            let dao = setup_dao_with_loading(&config)?;
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

#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {
    #[error(transparent)]
    ConfigLoad(#[from] ConfigLoadError),

    #[error(transparent)]
    AtCoder(#[from] atcoder::Error),

    #[error(transparent)]
    File(#[from] file_handler::Error),

    #[error(transparent)]
    Login(#[from] usecase::login::Error),

    #[error(transparent)]
    FetchTestSuite(#[from] usecase::fetch_test_suite::Error),

    #[error(transparent)]
    Test(#[from] usecase::test::Error),

    #[error("{0} command is not implemented yet")]
    Unimplemented(&'static str),
}
