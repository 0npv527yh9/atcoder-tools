use cookie_store::{Cookie, CookieStore};
use percent_encoding::percent_decode_str;

pub trait IntoCookieStore {
    fn into_cookie_store(self) -> CookieStore;
}

impl IntoCookieStore for Vec<Cookie<'static>> {
    fn into_cookie_store(self) -> CookieStore {
        let cookies = self.into_iter().map(Ok::<_, ()>);
        CookieStore::from_cookies(cookies, false).unwrap()
    }
}

pub fn parse_request_cookie_header(
    header: &str,
    request_url: &str,
) -> Result<Vec<Cookie<'static>>, Error> {
    let request_url = ::url::Url::parse(request_url).map_err(|_| Error::InvalidRequestUrl)?;
    let mut cookie_store = CookieStore::new(None);
    let mut parsed_count = 0;

    for cookie in header
        .split(';')
        .map(str::trim)
        .filter(|cookie| !cookie.is_empty())
    {
        if !is_request_cookie_pair(cookie) {
            return Err(Error::InvalidCookie(cookie.to_string()));
        }

        cookie_store
            .parse(cookie, &request_url)
            .map_err(|_| Error::InvalidCookie(cookie.to_string()))?;
        parsed_count += 1;
    }

    let cookies = cookie_store.iter_unexpired().cloned().collect::<Vec<_>>();
    if parsed_count == 0 || cookies.is_empty() {
        return Err(Error::NoCookies);
    }

    Ok(cookies)
}

pub fn extract_csrf_token(cookies: &[Cookie<'static>]) -> Option<String> {
    cookies
        .iter()
        .find(|cookie| cookie.name() == "csrf_token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            cookies
                .iter()
                .find(|cookie| cookie.name() == "REVEL_SESSION")
                .and_then(|cookie| extract_revel_session_csrf_token(cookie.value()))
        })
}

fn is_request_cookie_pair(cookie: &str) -> bool {
    let Some((name, value)) = cookie.split_once('=') else {
        return false;
    };

    !name.trim().is_empty() && !value.trim().is_empty()
}

fn extract_revel_session_csrf_token(value: &str) -> Option<String> {
    let decoded = percent_decode_str(value).decode_utf8().ok()?;
    let token = decoded.split("csrf_token:").nth(1)?;
    let token = token.split(['\0', '\n', '\r', '\t', ' ']).next()?;

    if token.is_empty() {
        None
    } else {
        Some(token.to_string())
    }
}

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("Invalid request URL")]
    InvalidRequestUrl,

    #[error("Invalid cookie: {0}")]
    InvalidCookie(String),

    #[error("No cookies found")]
    NoCookies,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    trait IntoCookies {
        fn into_cookies(self) -> Vec<Cookie<'static>>;
    }

    impl IntoCookies for CookieStore {
        fn into_cookies(self) -> Vec<Cookie<'static>> {
            self.iter_unexpired().cloned().collect()
        }
    }

    #[test]
    fn test_into_cookie_store() {
        let session_data = utils::test::load_session_data();

        let cookies_store = session_data.cookies.clone().into_cookie_store();
        let cookies = cookies_store.into_cookies();

        assert_eq!(session_data.cookies, cookies);
    }

    #[test]
    fn parse_request_cookie_header_parses_multiple_cookies() {
        let cookies = parse_request_cookie_header(
            "REVEL_SESSION=session-value; csrf_token=csrf-value",
            "https://atcoder.jp/",
        )
        .unwrap();

        assert_eq!(2, cookies.len());
        assert!(cookies
            .iter()
            .any(|cookie| cookie.name() == "REVEL_SESSION" && cookie.value() == "session-value"));
        assert!(cookies
            .iter()
            .any(|cookie| cookie.name() == "csrf_token" && cookie.value() == "csrf-value"));
    }

    #[test]
    fn parse_request_cookie_header_trims_spaces() {
        let cookies = parse_request_cookie_header(
            " REVEL_SESSION=session-value ; csrf_token=csrf-value ",
            "https://atcoder.jp/",
        )
        .unwrap();

        assert_eq!(2, cookies.len());
    }

    #[test]
    fn parse_request_cookie_header_rejects_empty_input() {
        let error = parse_request_cookie_header(" ", "https://atcoder.jp/").unwrap_err();

        assert_eq!(Error::NoCookies, error);
    }

    #[test]
    fn parse_request_cookie_header_rejects_invalid_fragment() {
        let error = parse_request_cookie_header(
            "REVEL_SESSION=session-value; invalid",
            "https://atcoder.jp/",
        )
        .unwrap_err();

        assert_eq!(Error::InvalidCookie("invalid".to_string()), error);
    }

    #[test]
    fn parse_request_cookie_header_rejects_empty_value() {
        let error = parse_request_cookie_header("csrf_token=", "https://atcoder.jp/").unwrap_err();

        assert_eq!(Error::InvalidCookie("csrf_token=".to_string()), error);
    }

    #[test]
    fn extract_csrf_token_from_csrf_cookie() {
        let cookies = parse_request_cookie_header(
            "REVEL_SESSION=session-value; csrf_token=csrf-value",
            "https://atcoder.jp/",
        )
        .unwrap();

        assert_eq!(Some("csrf-value".to_string()), extract_csrf_token(&cookies));
    }

    #[test]
    fn extract_csrf_token_from_revel_session_cookie() {
        let cookies = parse_request_cookie_header(
            "REVEL_SESSION=session%00csrf_token%3Arevel-csrf%00other%3Avalue",
            "https://atcoder.jp/",
        )
        .unwrap();

        assert_eq!(Some("revel-csrf".to_string()), extract_csrf_token(&cookies));
    }

    #[test]
    fn extract_csrf_token_returns_none_when_missing() {
        let cookies =
            parse_request_cookie_header("REVEL_SESSION=session-value", "https://atcoder.jp/")
                .unwrap();

        assert_eq!(None, extract_csrf_token(&cookies));
    }
}
