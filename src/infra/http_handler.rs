use crate::infra::atcoder::{html::Html, url::Url};
use cookie_store::Cookie;
use std::str::FromStr;
use ureq::{http::Uri, Agent};

const ATCODER_ORIGIN: &str = "https://atcoder.jp/";

pub struct HttpHandler {
    agent: Agent,
}

impl HttpHandler {
    pub fn new(agent: Agent) -> Self {
        Self { agent }
    }

    pub fn with_cookies(cookies: Vec<Cookie<'static>>) -> Self {
        let agent = Agent::new_with_defaults();
        let uri = atcoder_origin_uri();
        {
            let mut jar = agent.cookie_jar_lock();
            for cookie in cookies {
                if let Ok(cookie) = ureq::Cookie::parse(cookie.to_string(), &uri) {
                    let _ = jar.insert(cookie, &uri);
                }
            }
        }

        Self { agent }
    }

    pub fn get<PageType>(&self, url: &Url<PageType>) -> Result<Html<PageType>, Error> {
        let mut response = self.agent.get(url.as_str()).call()?;
        let html = response.body_mut().read_to_string()?.replace("\r", "");
        Ok(html.into())
    }

    pub fn post<'a, RequestPageType, ResponsePageType>(
        &self,
        url: &Url<RequestPageType>,
        data: impl Into<Vec<(&'static str, &'a str)>>,
    ) -> Result<Html<ResponsePageType>, Error> {
        let mut response = self.agent.post(url.as_str()).send_form(data.into())?;
        let html = response
            .body_mut()
            .read_to_string()?
            .replace("\r", "")
            .into();
        Ok(html)
    }

    pub fn into_cookies(self) -> Vec<Cookie<'static>> {
        let request_url = ::url::Url::parse(ATCODER_ORIGIN).unwrap();

        self.agent
            .cookie_jar_lock()
            .iter()
            .filter_map(|cookie| {
                Some(
                    Cookie::parse(
                        format!("{}={}", cookie.name(), cookie.value()),
                        &request_url,
                    )
                    .ok()?
                    .into_owned(),
                )
            })
            .collect()
    }
}

fn atcoder_origin_uri() -> Uri {
    Uri::from_str(ATCODER_ORIGIN).unwrap()
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
    use crate::{infra::atcoder::page_type, utils};

    #[test]
    fn with_cookies_round_trips_cookie_names_and_values() {
        let request_url = ::url::Url::parse(ATCODER_ORIGIN).unwrap();
        let cookies = vec![
            Cookie::parse(
                "REVEL_SESSION=session-value; Path=/; HttpOnly; Secure",
                &request_url,
            )
            .unwrap()
            .into_owned(),
            Cookie::parse("csrf_token=csrf-value; Path=/", &request_url)
                .unwrap()
                .into_owned(),
        ];

        let round_tripped = HttpHandler::with_cookies(cookies).into_cookies();

        assert!(round_tripped.iter().any(|cookie| {
            cookie.name() == "REVEL_SESSION" && cookie.value() == "session-value"
        }));
        assert!(round_tripped
            .iter()
            .any(|cookie| cookie.name() == "csrf_token" && cookie.value() == "csrf-value"));
    }

    #[test]
    #[ignore]
    fn test_get() {
        // Setup
        let expected = utils::test::load_homepage_html().html();
        let expected = expected.split('\n').collect::<Vec<_>>();

        let http_handler = HttpHandler {
            agent: Agent::new_with_defaults(),
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
