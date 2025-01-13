use crate::{
    dto::SessionData,
    handler::{
        http_handler::{self, HttpHandler},
        terminal_handler::Credentials,
    },
    parser::html_parser::HtmlParser,
};
use dto::LoginData;
use scraper::Html;

pub struct Dao {
    http_handler: HttpHandler,
    csrf_token: String,
}

impl Dao {
    pub fn new(http_handler: HttpHandler, csrf_token: String) -> Self {
        Self {
            http_handler,
            csrf_token,
        }
    }

    pub fn fetch_csrf_token(http_handler: &HttpHandler, url: &str) -> Result<String, Error> {
        let html = http_handler.get(url)?;
        Html::parse_document(&html)
            .csrf_token()
            .ok_or(Error::CsrfTokenNotFound)
    }

    pub fn login(
        &self,
        Credentials { username, password }: Credentials,
        url: &str,
    ) -> Result<(), Error> {
        let login_data = LoginData {
            username: &username,
            password: &password,
            csrf_token: &self.csrf_token,
        };

        let response = self.http_handler.post(url, login_data)?;

        let html = response
            .into_string()
            .map_err(|_| Error::Others("Too Large Response".to_string()))?;
        let html = Html::parse_document(&html);
        match html.title() {
            Some(title) if title == "AtCoder" => Ok(()),
            Some(_) => Err(Error::LoginFailed),
            None => Err(Error::Others("<title> Not Found".to_string())),
        }
    }

    pub fn into_session_data(self) -> SessionData {
        SessionData {
            cookies: self.http_handler.into_cookies(),
            csrf_token: self.csrf_token,
        }
    }
}

mod dto {
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct LoginData<'a> {
        pub username: &'a str,
        pub password: &'a str,
        pub csrf_token: &'a str,
    }

    impl<'a> From<LoginData<'a>> for Vec<(&str, &'a str)> {
        fn from(
            LoginData {
                username,
                password,
                csrf_token,
            }: LoginData<'a>,
        ) -> Self {
            vec![
                ("username", username),
                ("password", password),
                ("csrf_token", csrf_token),
            ]
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("CSRF Token Not Found")]
    CsrfTokenNotFound,

    #[error(transparent)]
    HttpHandler(#[from] http_handler::Error),

    #[error("Login Failed")]
    LoginFailed,

    #[error("{}", .0)]
    Others(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use ureq::Agent;

    #[test]
    #[ignore]
    fn test_login() {
        // Setup
        let http_handler = HttpHandler::new(Agent::new());
        let url = "https://atcoder.jp/login";
        let csrf_token = Dao::fetch_csrf_token(&http_handler, url).unwrap();
        let dao = Dao::new(http_handler, csrf_token);

        let username = rprompt::prompt_reply("username:").unwrap();
        let password = rpassword::prompt_password("password:").unwrap();

        // Run
        let response = dao.login(Credentials { username, password }, url);

        // Verify
        assert!(response.is_ok())
    }
}
