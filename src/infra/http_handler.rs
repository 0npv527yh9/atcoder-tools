use crate::infra::atcoder::{html::Html, url::Url};
use cookie_store::CookieStore;
use ureq::Agent;

pub struct HttpHandler {
    agent: Agent,
}

impl HttpHandler {
    pub fn new(agent: Agent) -> Self {
        Self { agent }
    }

    pub fn with_cookie_store(cookie_store: CookieStore) -> Result<Self, Error> {
        let agent = Agent::new_with_defaults();
        {
            let mut buf = Vec::new();
            serde_json::to_writer(&mut buf, &cookie_store).unwrap();
            let mut jar = agent.cookie_jar_lock();
            jar.load_json(&*buf)?;
        }
        Ok(Self { agent })
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

    pub fn cookie_store(&self) -> Result<CookieStore, Error> {
        let mut buf = Vec::new();

        self.agent.cookie_jar_lock().save_json(&mut buf);

        Ok(serde_json::from_slice(&buf).unwrap())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("HTTP Error: {:?}", .0)]
    HttpError(#[source] Box<ureq::Error>),

    #[error("Too Large Response")]
    TooLargeResponse(#[from] std::io::Error),

    #[error("Invalid cookie origin URL")]
    InvalidCookieOriginUrl,

    #[error("Cookie origin URL is not configured")]
    CookieOriginUrlNotConfigured,

    #[error("Invalid cookie: {0}")]
    InvalidCookie(String),
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

    const ORIGIN_URL: &str = "https://example.test/";

    #[test]
    fn with_cookie_store_round_trips_cookie_name_and_value() {
        let request_url: url::Url = ::url::Url::parse(ORIGIN_URL).unwrap();
        let mut cookie_store = CookieStore::default();
        cookie_store
            .parse(
                "REVEL_SESSION=session-value; Path=/; Expires=Thu, 31 Dec 2099 23:59:59 GMT; HttpOnly; Secure",
                &request_url,
            )
            .unwrap();

        let round_tripped = HttpHandler::with_cookie_store(cookie_store)
            .unwrap()
            .cookie_store()
            .unwrap();

        assert!(round_tripped.iter_any().any(|cookie| {
            cookie.name() == "REVEL_SESSION" && cookie.value() == "session-value"
        }));
    }

    #[test]
    #[ignore]
    fn test_get() {
        // Setup
        let expected = utils::test::load_homepage_html().html();
        let expected = expected.split('\n').collect::<Vec<_>>();

        let http_handler = HttpHandler::new(Agent::new_with_defaults());

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
