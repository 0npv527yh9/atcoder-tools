use ::time::OffsetDateTime;
use cookie_store::{Cookie, CookieStore};
use percent_encoding::percent_decode_str;
use serde::{Deserialize, Serialize};
use url::Url;

const REVEL_SESSION_COOKIE_NAME: &str = "REVEL_SESSION";
const REQUEST_URL: &str = "https://atcoder.jp/";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RevelSessionCookie {
    cookie: Cookie<'static>,
}

impl RevelSessionCookie {
    pub fn parse(cookie_str: &str) -> Result<Self, Error> {
        let cookie = Cookie::parse(cookie_str, &request_url())
            .unwrap()
            .into_owned();

        Ok(Self { cookie })
    }

    pub fn from_cookie_store(cookie_store: &CookieStore) -> Result<Self, Error> {
        let cookie = cookie_store
            .iter_any()
            .find(|cookie| cookie.name() == REVEL_SESSION_COOKIE_NAME)
            .cloned()
            .ok_or(Error::RevelSessionCookieNotFound)?;

        Ok(Self { cookie })
    }

    pub fn into_cookie_store(self) -> Result<CookieStore, Error> {
        let mut cookie_store = CookieStore::default();
        cookie_store.insert(self.cookie, &request_url()).unwrap();
        Ok(cookie_store)
    }

    pub fn csrf_token(&self) -> Option<String> {
        let decoded = percent_decode_str(self.cookie.value()).decode_utf8().ok()?;
        let (_, csrf_token) = decoded.split_once("csrf_token:")?;
        let csrf_token = csrf_token.split_whitespace().next()?;

        if csrf_token.is_empty() {
            None
        } else {
            Some(csrf_token.to_string())
        }
    }

    pub fn expires_datetime(&self) -> Option<OffsetDateTime> {
        self.cookie.expires_datetime()
    }
}

fn request_url() -> Url {
    Url::parse(REQUEST_URL).expect(&format!("Invalid request URL: {REQUEST_URL}"))
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid request URL")]
    InvalidRequestUrl,

    #[error("Invalid cookie: {0}")]
    InvalidCookie(#[from] cookie_store::Error),

    #[error("Unexpected cookie name: {0}")]
    UnexpectedCookieName(String),

    #[error("REVEL_SESSION cookie not found")]
    RevelSessionCookieNotFound,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn revel_session_cookie_parses_set_cookie() {
        let cookie =
            RevelSessionCookie::parse("REVEL_SESSION=session-value; Path=/; HttpOnly; Secure")
                .unwrap();

        assert_eq!("REVEL_SESSION", cookie.cookie.name());
        assert_eq!("session-value", cookie.cookie.value());
    }

    #[test]
    fn revel_session_cookie_parses_cookie_with_expires() {
        let cookie = RevelSessionCookie::parse(
            "REVEL_SESSION=session-value; Path=/; Expires=Thu, 31 Dec 2026 02:42:28 GMT; HttpOnly; Secure",
        )
        .unwrap();

        assert_eq!("session-value", cookie.cookie.value());
        assert!(cookie.expires_datetime().is_some());
    }

    #[test]
    fn revel_session_cookie_trims_spaces() {
        let cookie =
            RevelSessionCookie::parse(" REVEL_SESSION=session-value; Path=/; HttpOnly; Secure ")
                .unwrap();

        assert_eq!("session-value", cookie.cookie.value());
    }

    #[test]
    fn revel_session_cookie_rejects_invalid_url() {
        let error = RevelSessionCookie::parse("REVEL_SESSION=session-value; Path=/").unwrap_err();

        assert!(matches!(error, Error::InvalidRequestUrl));
    }

    #[test]
    fn revel_session_cookie_rejects_invalid_cookie() {
        let error = RevelSessionCookie::parse("invalid").unwrap_err();

        assert!(matches!(error, Error::InvalidCookie(_)));
    }

    #[test]
    fn revel_session_cookie_rejects_unexpected_cookie_name() {
        let error = RevelSessionCookie::parse("csrf_token=csrf-value; Path=/").unwrap_err();

        assert!(matches!(
            error,
            Error::UnexpectedCookieName(name) if name == "csrf_token"
        ));
    }

    #[test]
    fn revel_session_cookie_extracts_from_cookie_store() {
        let request_url = ::url::Url::parse("https://atcoder.jp/").unwrap();
        let mut cookie_store = CookieStore::default();
        cookie_store
            .parse("REVEL_SESSION=session-value; Path=/", &request_url)
            .unwrap();

        let cookie = RevelSessionCookie::from_cookie_store(&cookie_store).unwrap();

        assert_eq!("session-value", cookie.cookie.value());
    }

    #[test]
    fn revel_session_cookie_converts_to_cookie_store() {
        let cookie =
            RevelSessionCookie::parse("REVEL_SESSION=session-value; Path=/; HttpOnly; Secure")
                .unwrap();

        let cookie_store = cookie.into_cookie_store().unwrap();

        assert!(cookie_store
            .iter_any()
            .any(|cookie| cookie.name() == "REVEL_SESSION" && cookie.value() == "session-value"));
    }

    #[test]
    fn revel_session_cookie_extracts_csrf_token() {
        let cookie = RevelSessionCookie::parse(
            "REVEL_SESSION=session%00csrf_token%3Arevel-csrf%00other%3Avalue; Path=/",
        )
        .unwrap();

        assert_eq!(Some("revel-csrf".to_string()), cookie.csrf_token());
    }

    #[test]
    fn revel_session_cookie_returns_none_when_csrf_token_is_missing() {
        let cookie = RevelSessionCookie::parse("REVEL_SESSION=session-value; Path=/").unwrap();

        assert_eq!(None, cookie.csrf_token());
    }
}
