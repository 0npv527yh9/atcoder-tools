use crate::dto::cookie::RevelSessionCookie;

pub fn parse_imported_cookie(cookie_str: &str) -> Result<RevelSessionCookie, Error> {
    let revel_session_cookie = RevelSessionCookie::parse(cookie_str)?;
    Ok(revel_session_cookie)
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Cookie(#[from] crate::dto::cookie::Error),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dto::cookie;

    #[test]
    fn parse_imported_cookie_accepts_revel_session_cookie() {
        let cookie = parse_imported_cookie(
            "REVEL_SESSION=session%00csrf_token%3Arevel-csrf%00other%3Avalue; Path=/",
        )
        .unwrap();

        assert_eq!(Some("revel-csrf".to_string()), cookie.csrf_token());
    }

    #[test]
    fn parse_imported_cookie_rejects_invalid_cookie() {
        let error = parse_imported_cookie("invalid").unwrap_err();

        assert!(matches!(
            error,
            Error::Cookie(cookie::Error::InvalidCookie(_))
        ));
    }

    #[test]
    fn parse_imported_cookie_rejects_unexpected_cookie_name() {
        let error = parse_imported_cookie("csrf_token=csrf-value; Path=/").unwrap_err();

        assert!(matches!(
            error,
            Error::Cookie(cookie::Error::UnexpectedCookieName(name)) if name == "csrf_token"
        ));
    }
}
