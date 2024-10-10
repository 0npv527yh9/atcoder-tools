use cookie_store::{Cookie, CookieStore};
use serde::Serialize;

#[derive(Serialize)]
pub struct SessionData {
    pub cookies: Vec<Cookie<'static>>,
    pub csrf_token: String,
}

pub trait IntoCookieStore {
    fn into_cookie_store(self) -> CookieStore;
}

impl IntoCookieStore for Vec<Cookie<'static>> {
    fn into_cookie_store(self) -> CookieStore {
        let cookies = self.into_iter().map(Ok::<_, ()>);
        CookieStore::from_cookies(cookies, false).unwrap()
    }
}
