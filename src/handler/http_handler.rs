use crate::{
    dto::{cookie::IntoCookieStore, SessionData},
    parser::html_parser::HtmlParser,
};
use cookie_store::Cookie;
use scraper::Html;
use ureq::{serde::Serialize, Agent, Response};

pub struct HttpHandler {
    agent: Agent,
    pub csrf_token: String,
}

impl HttpHandler {
    fn with_fetching(url: &str) -> Result<Self, Error> {
        let agent = Agent::new();

        let response = agent.get(url).call().map_err(Error::HttpError)?;
        let html = response
            .into_string()
            .map_err(|_| Error::TooLargeResponse)?;

        let csrf_token = (&Html::parse_document(&html))
            .csrf_token()
            .ok_or(Error::CsrfTokenNotFound)?;

        Ok(Self { agent, csrf_token })
    }

    fn with_session_data(
        SessionData {
            cookies,
            csrf_token,
        }: SessionData,
    ) -> Self {
        let cookie_store = cookies.into_cookie_store();
        let agent = ureq::builder().cookie_store(cookie_store).build();
        Self { agent, csrf_token }
    }

    fn get(&self, url: &str) -> Result<String, Error> {
        let response = self.agent.get(url).call().map_err(Error::HttpError)?;
        response.into_string().map_err(|_| Error::TooLargeResponse)
    }

    pub fn post(&self, url: &str, data: impl Serialize) -> Result<Response, ureq::Error> {
        self.agent.post(url).send_json(data)
    }

    fn session_data(&self) -> SessionData {
        SessionData {
            cookies: self.cookies(),
            csrf_token: self.csrf_token.clone(),
        }
    }

    fn cookies(&self) -> Vec<Cookie<'static>> {
        self.agent
            .cookie_store()
            .iter_unexpired()
            .cloned()
            .collect()
    }
}

#[derive(Debug)]
enum Error {
    HttpError(ureq::Error),
    CsrfTokenNotFound,
    TooLargeResponse,
}

#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;
    use std::fs;

    #[test]
    #[ignore]
    fn test_get() {
        // Setup
        let url = std::env::var("URL")
            .expect("You should set the target `URL` as an environment variable.");
        let expected_file = utils::test::load_homepage_html();

        let expected = fs::read_to_string(expected_file).unwrap();
        let expected = expected.trim().split('\n').collect::<Vec<_>>();

        // Run
        let http_handler = HttpHandler {
            agent: Agent::new(),
            csrf_token: String::from("Dummy CSRF Token"),
        };
        let actual = http_handler.get(&url).unwrap();
        let actual = actual.replace("\r", "");
        let actual = actual.trim().split('\n').collect::<Vec<_>>();

        // Verify
        assert_eq!(expected.len(), actual.len());
        for (expected, actual) in expected.iter().zip(actual.iter()) {
            if !actual.contains("csrf_token") && !actual.contains("csrfToken") {
                assert_eq!(expected, actual);
            }
        }
    }

    #[test]
    #[ignore]
    fn test_with_fetching() {
        // Setup
        let url = std::env::var("URL")
            .expect("You should set the target `URL` as an environment variable.");

        // Run
        let http_handler = HttpHandler::with_fetching(&url);

        // Verify
        assert!(http_handler.is_ok())
    }

    #[test]
    fn test_with_session_data() {
        // Setup
        let session_data = utils::test::load_session_data();

        // Execute
        let http_handler = HttpHandler::with_session_data(session_data);

        // Verify
        let expected = utils::test::load_session_data();
        let actual = http_handler.session_data();
        assert_eq!(
            serde_json::to_string(&expected).unwrap(),
            serde_json::to_string(&actual).unwrap()
        );
    }
}
