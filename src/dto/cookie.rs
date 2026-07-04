use cookie_store::{Cookie, CookieStore};
use percent_encoding::percent_decode_str;

pub fn parse_request_cookie_header(
    header: &str,
    request_url: &str,
) -> Result<Vec<Cookie<'static>>, Error> {
    let request_url = ::url::Url::parse(request_url).map_err(|_| Error::InvalidRequestUrl)?;
    let mut cookie_store = CookieStore::new(None);

    for cookie in request_cookie_pairs(header) {
        let cookie = cookie?;
        cookie_store
            .parse(cookie, &request_url)
            .map_err(|_| Error::InvalidCookie(cookie.to_string()))?;
    }

    let cookies = cookie_store.iter_unexpired().cloned().collect::<Vec<_>>();
    if cookies.is_empty() {
        return Err(Error::NoCookies);
    }

    Ok(cookies)
}

pub fn extract_csrf_token(cookies: &[Cookie<'static>]) -> Option<String> {
    extract_csrf_cookie_token(cookies).or_else(|| extract_revel_session_cookie_token(cookies))
}

fn request_cookie_pairs(header: &str) -> impl Iterator<Item = Result<&str, Error>> {
    header
        .split(';')
        .map(str::trim)
        .filter(|cookie| !cookie.is_empty())
        .map(validate_request_cookie_pair)
}

fn validate_request_cookie_pair(cookie: &str) -> Result<&str, Error> {
    let Some((name, value)) = cookie.split_once('=') else {
        return Err(Error::InvalidCookie(cookie.to_string()));
    };

    if name.trim().is_empty() || value.trim().is_empty() {
        return Err(Error::InvalidCookie(cookie.to_string()));
    }

    Ok(cookie)
}

fn extract_csrf_cookie_token(cookies: &[Cookie<'static>]) -> Option<String> {
    cookies
        .iter()
        .find(|cookie| cookie.name() == "csrf_token")
        .map(|cookie| cookie.value().to_string())
}

fn extract_revel_session_cookie_token(cookies: &[Cookie<'static>]) -> Option<String> {
    cookies
        .iter()
        .find(|cookie| cookie.name() == "REVEL_SESSION")
        .and_then(|cookie| extract_revel_session_csrf_token(cookie.value()))
}

fn extract_revel_session_csrf_token(value: &str) -> Option<String> {
    let decoded = percent_decode_str(value).decode_utf8().ok()?;
    let (_, token) = decoded.split_once("csrf_token:")?;
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
