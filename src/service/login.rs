use crate::{
    dao::{self, Dao},
    domain::{page_type, url::Url},
    handler::terminal_handler,
};
use std::fs;

pub struct LoginService {
    dao: Dao,
}

impl LoginService {
    pub fn new(dao: Dao) -> Self {
        Self { dao }
    }

    pub fn login(&self, url: &Url<page_type::Login>) -> Result<(), Error> {
        let credentials = terminal_handler::read_credentials().map_err(Error::Terminal)?;

        self.dao.login(credentials, url).or_else(|error| {
            let should_retry = terminal_handler::ask_for_retry().map_err(Error::Terminal)?;
            if should_retry {
                self.login(url)
            } else {
                Err(Error::Dao(error))
            }
        })
    }

    pub fn save_session_data(self, file_path: &str) -> Result<(), Error> {
        let session_data = self.dao.into_session_data();

        let contents = serde_json::to_string_pretty(&session_data)
            .map_err(|_| Error::Others("Session Data Serialization Failed".to_string()))?;

        fs::write(file_path, contents)
            .map_err(|_| Error::Others("Failed to save session data".to_string()))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Dao(#[from] dao::Error),

    #[error("Terminal Input Error: {:?}", .0)]
    Terminal(#[source] std::io::Error),

    #[error("{}", .0)]
    Others(String),
}
