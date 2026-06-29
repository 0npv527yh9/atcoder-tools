pub mod html;
pub mod page_type;
pub mod url;
use self::url::Url;
use crate::{
    dto::{SessionData, TestSuite},
    infra::http_handler::{self, HttpHandler},
};

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

    pub fn fetch_test_suite(&self, url: &Url<page_type::Task>) -> Result<TestSuite, Error> {
        let html = self.http_handler.get(url)?;
        Ok(html.test_suite())
    }

    pub fn fetch_task_screen_names(
        &self,
        tasks_url: &Url<page_type::Tasks>,
    ) -> Result<Vec<String>, Error> {
        let html = self.http_handler.get(tasks_url)?;
        Ok(html.task_screen_names())
    }

    pub fn into_session_data(self) -> SessionData {
        SessionData {
            cookies: self.http_handler.into_cookies(),
            csrf_token: self.csrf_token,
        }
    }

    pub fn check_login(&self, url: &Url<page_type::Home>) -> Result<bool, Error> {
        let html = self.http_handler.get(url)?;
        Ok(html.is_logged_in())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    HttpHandler(#[from] http_handler::Error),
}

#[cfg(test)]
mod tests {
    use super::*;
    use ureq::Agent;

    #[test]
    #[ignore]
    fn test_fetch_test_suite_task_page() {
        // Setup
        let http_handler = HttpHandler::new(Agent::new());
        let task_url = "https://atcoder.jp/contests/abc388/tasks/abc388_a"
            .to_string()
            .into();
        let dao = Dao::new(http_handler, "Dummy CSRF Token".to_string());

        // Run
        let test_suite = dao.fetch_test_suite(&task_url).unwrap();

        // Verify
        println!("{test_suite:#?}");
        assert_eq!(1, test_suite.len());
        assert_eq!(2, test_suite[0].test_cases.len());
    }

    #[test]
    #[ignore]
    fn test_fetch_test_suite_tasks_print() {
        // Setup
        let http_handler = HttpHandler::new(Agent::new());
        let tasks_print_url = "https://atcoder.jp/contests/abc388/tasks_print"
            .to_string()
            .into();
        let dao = Dao::new(http_handler, "Dummy CSRF Token".to_string());

        // Run
        let test_suite = dao.fetch_test_suite(&tasks_print_url).unwrap();

        // Verify
        println!("{test_suite:#?}");
        assert_eq!(7, test_suite.len());
    }

    #[test]
    #[ignore]
    fn test_fetch_task_screen_names() {
        // Setup
        let http_handler = HttpHandler::new(Agent::new());
        let tasks_url = "https://atcoder.jp/contests/abc388/tasks"
            .to_string()
            .into();
        let dao = Dao::new(http_handler, "Dummy CSRF Token".to_string());

        // Run
        let task_screen_names = dao.fetch_task_screen_names(&tasks_url).unwrap();

        // Verify
        assert_eq!(
            vec![
                "abc388_a", "abc388_b", "abc388_c", "abc388_d", "abc388_e", "abc388_f", "abc388_g"
            ],
            task_screen_names
        );
    }
}
