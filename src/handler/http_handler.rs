use crate::dto::cookie::IntoCookieStore;
use anyhow::Result;
use cookie_store::Cookie;
use ureq::{Agent, Response};

pub struct HttpHandler {
    agent: Agent,
}

impl HttpHandler {
    pub fn new(agent: Agent) -> Self {
        Self { agent }
    }

    fn with_cookies(cookies: Vec<Cookie<'static>>) -> Self {
        let cookie_store = cookies.into_cookie_store();
        let agent = ureq::builder().cookie_store(cookie_store).build();
        Self { agent }
    }

    pub fn get(&self, url: &str) -> Result<String> {
        let response = self.agent.get(url).call()?;
        Ok(response.into_string()?.replace("\r", ""))
    }

    pub fn post<'a>(
        &self,
        url: &str,
        data: impl Into<Vec<(&'static str, &'a str)>>,
    ) -> Result<Response> {
        Ok(self.agent.post(url).send_form(&data.into())?)
    }

    pub fn into_cookies(self) -> Vec<Cookie<'static>> {
        self.agent
            .cookie_store()
            .iter_unexpired()
            .cloned()
            .collect()
    }
}

#[derive(Debug)]
pub enum Error {
    HttpError(ureq::Error),
    CsrfTokenNotFound,
    TooLargeResponse,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    #[ignore]
    fn test_get() {
        // Setup
        let expected = utils::test::load_homepage_html();
        let expected = expected.split('\n').collect::<Vec<_>>();

        let http_handler = HttpHandler {
            agent: Agent::new(),
        };

        // Run
        let url = "https://atcoder.jp/home";
        let actual = http_handler.get(url).unwrap();
        let actual = actual.replace("\r", "");
        let actual = actual.split('\n').collect::<Vec<_>>();

        // Verify
        assert_eq!(expected.len(), actual.len());
        for (expected, actual) in expected.into_iter().zip(actual.into_iter()) {
            if !(actual.contains("csrf") || actual.contains("fixtime")) {
                assert_eq!(expected, actual);
            }
        }
    }
}
