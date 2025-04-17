pub mod config;
pub mod cookie;

use crate::domain::{page_type, url::Url};
use cookie_store::Cookie;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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
    pub working_dir: Option<PathBuf>,
}

pub struct Diff {
    pub input: String,
    pub expected: String,
    pub actual: String,
    pub file: String,
}
