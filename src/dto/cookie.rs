use cookie_store::{Cookie, CookieStore};

pub trait IntoCookieStore {
    fn into_cookie_store(self) -> CookieStore;
}

impl IntoCookieStore for Vec<Cookie<'static>> {
    fn into_cookie_store(self) -> CookieStore {
        let cookies = self.into_iter().map(Ok::<_, ()>);
        CookieStore::from_cookies(cookies, false).unwrap()
    }
}

#[cfg(test)]
trait IntoCookies {
    fn into_cookies(self) -> Vec<Cookie<'static>>;
}

#[cfg(test)]
impl IntoCookies for CookieStore {
    fn into_cookies(self) -> Vec<Cookie<'static>> {
        self.iter_unexpired().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn test_into_cookie_store() {
        let session_data = utils::test::load_session_data();

        let cookies_store = session_data.cookies.clone().into_cookie_store();
        let cookies = cookies_store.into_cookies();

        assert_eq!(session_data.cookies, cookies);
    }
}
