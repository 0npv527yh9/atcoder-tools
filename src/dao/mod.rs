use crate::{
    dto::SessionData,
    handler::{http_handler::HttpHandler, terminal_handler::Credentials},
    parser::html_parser::HtmlParser,
};
use anyhow::{anyhow, Result};
use dto::LoginData;
use scraper::Html;
use ureq::Response;

pub struct Dao {
    http_handler: HttpHandler,
    csrf_token: String,
}

impl Dao {
    pub fn with_fetching(http_handler: HttpHandler, url: &str) -> Result<Self> {
        let html = http_handler.get(url)?;
        let csrf_token = (&Html::parse_document(&html))
            .csrf_token()
            .ok_or(anyhow!("CSRF Token Not Found"))?;

        Ok(Self {
            http_handler,
            csrf_token,
        })
    }

    pub fn login(
        &self,
        Credentials { username, password }: Credentials,
        url: &str,
    ) -> Result<Response> {
        let login_data = LoginData {
            username: &username,
            password: &password,
            csrf_token: &self.csrf_token,
        };
        self.http_handler.post(url, login_data)
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

    impl<'a> From<LoginData<'a>> for [(&str, &'a str); 3] {
        fn from(
            LoginData {
                username,
                password,
                csrf_token,
            }: LoginData<'a>,
        ) -> Self {
            [
                ("username", username),
                ("password", password),
                ("csrf_token", csrf_token),
            ]
        }
    }
}
