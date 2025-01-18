use crate::{
    dto::{SessionData, TestCases},
    handler::{
        http_handler::{self, HttpHandler},
        terminal_handler::Credentials,
    },
    parser::html_parser::{HtmlParser, TasksHtml},
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

    pub fn fetch_test_suites(&self, url: &str) -> Result<Vec<TestCases>, Error> {
        let html = self.http_handler.get(url)?;
        Ok(Html::parse_document(&html).test_suites())
    }

    pub fn fetch_task_screen_names(&self, tasks_url: &str) -> Result<Vec<String>, Error> {
        let html = self.http_handler.get(tasks_url)?;
        let html = Html::parse_document(&html);
        let html = TasksHtml(html);
        Ok(html.task_screen_names())
    }

    pub fn into_session_data(self) -> SessionData {
        SessionData {
            cookies: self.http_handler.into_cookies(),
            csrf_token: self.csrf_token,
        }
    }
}

pub mod dto {
    use serde::Serialize;

    #[derive(Serialize)]
    pub(super) struct LoginData<'a> {
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

    #[test]
    #[ignore]
    fn test_fetch_test_suites_task_page() {
        // Setup
        let http_handler = HttpHandler::new(Agent::new());
        let task_url = "https://atcoder.jp/contests/abc388/tasks/abc388_a";
        let dao = Dao::new(http_handler, "Dummy CSRF Token".to_string());

        // Run
        let test_suites = dao.fetch_test_suites(task_url).unwrap();

        // Verify
        println!("{test_suites:#?}");
        assert_eq!(1, test_suites.len());
        assert_eq!(2, test_suites[0].test_cases.len());
    }

    #[test]
    #[ignore]
    fn test_fetch_test_suites_tasks_print() {
        // Setup
        let http_handler = HttpHandler::new(Agent::new());
        let tasks_print_url = "https://atcoder.jp/contests/abc388/tasks_print";
        let dao = Dao::new(http_handler, "Dummy CSRF Token".to_string());

        // Run
        let test_suites = dao.fetch_test_suites(tasks_print_url).unwrap();

        // Verify
        println!("{test_suites:#?}");
        assert_eq!(7, test_suites.len());
    }

    #[test]
    #[ignore]
    fn test_fetch_task_screen_names() {
        // Setup
        let http_handler = HttpHandler::new(Agent::new());
        let tasks_url = "https://atcoder.jp/contests/abc388/tasks";
        let dao = Dao::new(http_handler, "Dummy CSRF Token".to_string());

        // Run
        let task_screen_names = dao.fetch_task_screen_names(tasks_url).unwrap();

        // Verify
        assert_eq!(
            vec![
                "abc388_a", "abc388_b", "abc388_c", "abc388_d", "abc388_e", "abc388_f", "abc388_g"
            ],
            task_screen_names
        );
    }
}
