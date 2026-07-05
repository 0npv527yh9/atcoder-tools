pub mod config;
pub mod cookie;

use crate::dto::cookie::RevelSessionCookie;
use crate::infra::atcoder::{page_type, url::Url};
use ::time::OffsetDateTime;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct SessionData {
    pub revel_session_cookie: RevelSessionCookie,
}

impl SessionData {
    pub fn expired_datetime(&self) -> Option<OffsetDateTime> {
        self.revel_session_cookie.expires_datetime()
    }
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

pub type TestSuite = Vec<TestCases>;

#[derive(Debug)]
pub struct TestCaseFile {
    pub test_case: TestCase,
    pub file: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskInfo {
    pub task: String,
    pub contest_url: Url<page_type::ContestHome>,
    pub task_screen_name: String,
}

#[derive(Deserialize)]
pub struct Command {
    pub command: String,
    pub args: Vec<String>,
    #[serde(default = "default_working_dir")]
    pub working_dir: PathBuf,
}

fn default_working_dir() -> PathBuf {
    PathBuf::from(".")
}

pub struct Diff {
    pub input: String,
    pub expected: String,
    pub actual: String,
    pub file: String,
}
