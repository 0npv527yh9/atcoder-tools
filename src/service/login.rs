use crate::handler::http_handler::{self, HttpHandler};
use dto::LoginData;
use std::fs;
use ureq::Response;

pub struct LoginService {
    pub handler: HttpHandler,
    pub url: String,
}

impl LoginService {
    pub fn with_fetching(url: &str) -> Result<Self, Error> {
        let handler = HttpHandler::with_fetching(url).map_err(Error::HttpHandlerError)?;
        Ok(Self {
            handler,
            url: url.to_string(),
        })
    }

    pub fn login(&self, username: &str, password: &str) -> Result<Response, Error> {
        let login_data = LoginData {
            username,
            password,
            csrf_token: &self.handler.csrf_token,
        };
        self.handler
            .post(&self.url, &login_data.into_pairs())
            .map_err(|e| Error::HttpHandlerError(e))
    }

    pub fn save_session_data(self) -> Result<(), std::io::Error> {
        let session_data = self.handler.into_session_data();
        fs::write(
            "session_data.json",
            serde_json::to_string(&session_data).expect(""),
        )
    }
}

#[derive(Debug)]
pub enum Error {
    HttpHandlerError(http_handler::Error),
    IOError(std::io::Error),
}

mod dto {
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct LoginData<'a> {
        pub username: &'a str,
        pub password: &'a str,
        pub csrf_token: &'a str,
    }

    impl<'a> LoginData<'a> {
        pub fn into_pairs(self) -> [(&'a str, &'a str); 3] {
            let LoginData {
                username,
                password,
                csrf_token,
            } = self;
            [
                ("username", username),
                ("password", password),
                ("csrf_token", csrf_token),
            ]
        }
    }
}
