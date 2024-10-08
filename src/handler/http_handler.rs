use crate::parser::html_parser::HtmlParser;
use anyhow::Result;
use scraper::Html;
use std::fmt;
use ureq::{serde::Serialize, Agent, Response};

struct HttpHandler {
    agent: Agent,
    csrf_token: String,
}

impl HttpHandler {
    fn with_fetching(url: &str) -> Result<Self> {
        let agent = Agent::new();

        let response = agent.get(url).call()?;
        let html = response.into_string()?;

        let csrf_token = Html::parse_document(&html)
            .csrf_token()
            .ok_or(Error::CsrfTokenNotFound)?;

        Ok(Self { agent, csrf_token })
    }

    fn get(&self, url: &str) -> Result<String> {
        let response = self.agent.get(url).call()?;
        let html = response.into_string()?;
        Ok(html)
    }

    fn post(&self, url: &str, data: impl Serialize) -> Result<Response> {
        Ok(self.agent.post(url).send_json(data)?)
    }
}

#[derive(Debug)]
enum Error {
    CsrfTokenNotFound,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::CsrfTokenNotFound => write!(f, "CSRF Token Not Found"),
        }
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    #[ignore]
    fn test_get() {
        // Setup
        let url = std::env::var("URL")
            .expect("You should set the target `URL` as an environment variable.");
        let expected_file = std::env::var("EXPECTED_FILE")
            .expect("You should set the `EXPECTED_FILE` as an environment variable.");

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
}
