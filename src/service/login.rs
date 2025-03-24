use crate::{
    dao::{self, Dao},
    domain::{page_type, url::Url},
    handler::{file_handler, terminal_handler},
};
use std::path::Path;

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

    pub fn save_session_data(self, file_path: &Path) -> Result<(), Error> {
        let session_data = self.dao.into_session_data();
        file_handler::save(file_path, &session_data)?;
        Ok(())
    }
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
