pub mod cookie;

use cookie_store::Cookie;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SessionData {
    pub cookies: Vec<Cookie<'static>>,
    pub csrf_token: String,
}

#[derive(Debug)]
pub struct TestCase {
    pub input: String,
    pub output: String,
}

#[derive(Debug)]
pub struct TestCases {
    pub task: String,
    pub test_cases: Vec<TestCase>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskInfo {
    pub task: String,
    pub contest_url: String,
    pub task_screen_name: String,
}
