use ureq::{Error, Response};

use crate::handler::http_handler::HttpHandler;

struct LoginService {
    handler: HttpHandler,
    url: String,
}

impl LoginService {
    pub fn login(&self, username: &str, password: &str) -> Result<Response, Error> {
        let login_data = dto::LoginData {
            username,
            password,
            csrf_token: &self.handler.csrf_token,
        };
        self.handler.post(&self.url, login_data)
    }
}

mod dto {
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct LoginData<'a> {
        pub username: &'a str,
        pub password: &'a str,
        pub csrf_token: &'a str,
    }
}
