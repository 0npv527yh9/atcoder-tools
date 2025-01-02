pub mod cookie;

use cookie_store::Cookie;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SessionData {
    pub cookies: Vec<Cookie<'static>>,
    pub csrf_token: String,
}
