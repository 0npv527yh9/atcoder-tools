use crate::{
    domain::{html::Html, url::Url},
    dto::cookie::IntoCookieStore,
};
use cookie_store::Cookie;
use ureq::Agent;

pub struct HttpHandler {
    agent: Agent,
}

impl HttpHandler {
    pub fn new(agent: Agent) -> Self {
        Self { agent }
    }

    pub fn with_cookies(cookies: Vec<Cookie<'static>>) -> Self {
        let cookie_store = cookies.into_cookie_store();
        let agent = ureq::builder().cookie_store(cookie_store).build();
        Self { agent }
    }

    pub fn get<PageType>(&self, url: &Url<PageType>) -> Result<Html<PageType>, Error> {
        let response = self.agent.get(url).call()?;
        let html = response.into_string()?.replace("\r", "");
        Ok(html.into())
    }

    pub fn post<'a, RequestPageType, ResponsePageType>(
        &self,
        url: &Url<RequestPageType>,
        data: impl Into<Vec<(&'static str, &'a str)>>,
    ) -> Result<Html<ResponsePageType>, Error> {
        let response = self.agent.post(url).send_form(&data.into())?;
        let html = response.into_string()?.replace("\r", "").into();
        Ok(html)
    }

    pub fn into_cookies(self) -> Vec<Cookie<'static>> {
        self.agent
            .cookie_store()
            .iter_unexpired()
            .cloned()
            .collect()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("HTTP Error: {:?}", .0)]
    HttpError(#[source] Box<ureq::Error>),

    #[error("Too Large Response")]
    TooLargeResponse(#[from] std::io::Error),
}

impl From<ureq::Error> for Error {
    fn from(value: ureq::Error) -> Self {
        Error::HttpError(Box::new(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{domain::page_type, utils};

    #[test]
    #[ignore]
    fn test_get() {
        // Setup
        let expected = utils::test::load_homepage_html().html();
        let expected = expected.split('\n').collect::<Vec<_>>();

        let http_handler = HttpHandler {
            agent: Agent::new(),
        };

        // Run
        let url: Url<page_type::Home> = "https://atcoder.jp/home".to_string().into();
        let actual = http_handler.get(&url).unwrap().html();
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
