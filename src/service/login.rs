use crate::{dao::Dao, handler::terminal_handler};
use anyhow::Result;
use std::fs;

pub struct LoginService {
    dao: Dao,
}

impl LoginService {
    pub fn new(dao: Dao) -> Self {
        Self { dao }
    }

    pub fn login(&self, url: &str) -> Result<()> {
        loop {
            let credentials = terminal_handler::read_credentials()?;

            match self.dao.login(credentials, url) {
                Ok(_) => {
                    println!("Login Success");
                    return Ok(());
                }
                Err(_) => {
                    if !terminal_handler::ask_for_retry()? {
                        return Err(anyhow::anyhow!("Login Failed"));
                    }
                }
            }
        }
    }

    pub fn save(self, file_path: &str) -> Result<()> {
        self.save_session_data(file_path)
    }

    fn save_session_data(self, file_path: &str) -> Result<()> {
        let session_data = self.dao.into_session_data();
        Ok(fs::write(
            file_path,
            serde_json::to_string(&session_data).expect("Serialization failed"),
        )?)
    }
}

#[derive(Debug)]
pub enum Error {
    HttpHandlerError(http_handler::Error),
    IOError(std::io::Error),
}

