use super::page_type;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{marker::PhantomData, ops::Deref, str::FromStr};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(transparent)]
pub struct Url<PageType>(String, PhantomData<fn() -> PageType>);

impl<PageType> From<String> for Url<PageType> {
    fn from(url: String) -> Self {
        Self(url, PhantomData)
    }
}

impl<PageType> Deref for Url<PageType> {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone)]
pub enum FetchTaskUrl {
    Contest {
        contest_url: Url<page_type::ContestHome>,
        tasks_print_url: Url<page_type::Task>,
        tasks_url: Url<page_type::Tasks>,
    },
    Task {
        task_url: Url<page_type::Task>,
        contest_url: Url<page_type::ContestHome>,
        task_screen_name: String,
    },
}

impl FromStr for FetchTaskUrl {
    type Err = Error;

    fn from_str(url: &str) -> Result<Self, Self::Err> {
        let contest_homepage_pattern = Regex::new(r"^https://atcoder\.jp/contests/[^/]+$").unwrap();
        let task_page_pattern =
            Regex::new(r"^(https://atcoder\.jp/contests/[^/]+)/tasks/([^/]+)+$").unwrap();

        if contest_homepage_pattern.is_match(url) {
            let contest_url = url.to_string();
            Ok(FetchTaskUrl::Contest {
                tasks_print_url: format!("{contest_url}/tasks_print").into(),
                tasks_url: format!("{contest_url}/tasks").into(),
                contest_url: contest_url.into(),
            })
        } else if let Some(captures) = task_page_pattern.captures(url) {
            Ok(FetchTaskUrl::Task {
                task_url: url.to_string().into(),
                contest_url: captures[1].to_string().into(),
                task_screen_name: captures[2].to_string(),
            })
        } else {
            Err(Error::Parse(url.to_string()))
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to parse URL: {}", .0)]
    Parse(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_contest_homepage_url() {
        // Setup
        let contest_homepage_url = "https://atcoder.jp/contests/abc388";

        // Run
        let task_url: Result<FetchTaskUrl, Error> = contest_homepage_url.parse();

        // Verify
        if let Ok(FetchTaskUrl::Contest {
            contest_url,
            tasks_print_url,
            tasks_url,
        }) = task_url
        {
            assert_eq!("https://atcoder.jp/contests/abc388", *contest_url);
            assert_eq!(
                "https://atcoder.jp/contests/abc388/tasks_print",
                *tasks_print_url
            );
            assert_eq!("https://atcoder.jp/contests/abc388/tasks", *tasks_url);
        } else {
            unreachable!()
        }
    }

    #[test]
    fn test_parse_task_page_url() {
        // Setup
        let task_url = "https://atcoder.jp/contests/abc388/tasks/abc388_a";

        // Run
        let task_url: Result<FetchTaskUrl, Error> = task_url.parse();

        // Verify
        if let Ok(FetchTaskUrl::Task {
            task_url: url,
            contest_url,
            task_screen_name,
        }) = task_url
        {
            assert_eq!("https://atcoder.jp/contests/abc388/tasks/abc388_a", *url);
            assert_eq!("https://atcoder.jp/contests/abc388", *contest_url);
            assert_eq!("abc388_a", task_screen_name)
        } else {
            unreachable!()
        }
    }

    #[test]
    fn fail_with_invalid_url() {
        // Setup
        let url = "invalid-url";

        // Run
        let task_url: Result<FetchTaskUrl, Error> = url.parse();

        // Verify
        if let Err(error) = task_url {
            assert_eq!("Failed to parse URL: invalid-url", error.to_string())
        } else {
            unreachable!()
        }
    }
}
