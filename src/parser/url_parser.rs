use regex::Regex;
use std::str::FromStr;

#[derive(Clone)]
pub enum Url {
    Contest {
        contest_url: String,
        tasks_print_url: String,
        tasks_url: String,
    },
    Task {
        task_url: String,
        contest_url: String,
        task_screen_name: String,
    },
}

impl FromStr for Url {
    type Err = Error;

    fn from_str(url: &str) -> Result<Self, Self::Err> {
        let contest_homepage_pattern = Regex::new(r"^https://atcoder\.jp/contests/[^/]+$").unwrap();
        let task_page_pattern =
            Regex::new(r"^(https://atcoder\.jp/contests/[^/]+)/tasks/([^/]+)+$").unwrap();

        if contest_homepage_pattern.is_match(url) {
            let contest_url = url.to_string();
            Ok(Url::Contest {
                tasks_print_url: format!("{contest_url}/tasks_print"),
                tasks_url: format!("{contest_url}/tasks"),
                contest_url,
            })
        } else if let Some(captures) = task_page_pattern.captures(url) {
            Ok(Url::Task {
                task_url: url.to_string(),
                contest_url: captures[1].to_string(),
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
        let task_url: Result<Url, Error> = contest_homepage_url.parse();

        // Verify
        if let Ok(Url::Contest {
            contest_url,
            tasks_print_url,
            tasks_url,
        }) = task_url
        {
            assert_eq!("https://atcoder.jp/contests/abc388", contest_url);
            assert_eq!(
                "https://atcoder.jp/contests/abc388/tasks_print",
                tasks_print_url
            );
            assert_eq!("https://atcoder.jp/contests/abc388/tasks", tasks_url);
        } else {
            unreachable!()
        }
    }

    #[test]
    fn test_parse_task_page_url() {
        // Setup
        let task_url = "https://atcoder.jp/contests/abc388/tasks/abc388_a";

        // Run
        let task_url: Result<Url, Error> = task_url.parse();

        // Verify
        if let Ok(Url::Task {
            task_url: url,
            contest_url,
            task_screen_name,
        }) = task_url
        {
            assert_eq!("https://atcoder.jp/contests/abc388/tasks/abc388_a", url);
            assert_eq!("https://atcoder.jp/contests/abc388", contest_url);
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
        let task_url: Result<Url, Error> = url.parse();

        // Verify
        if let Err(error) = task_url {
            assert_eq!("Failed to parse URL: invalid-url", error.to_string())
        } else {
            unreachable!()
        }
    }
}
