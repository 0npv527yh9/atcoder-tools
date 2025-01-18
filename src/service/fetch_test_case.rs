use crate::{
    config::Config,
    dao::{self, Dao},
    dto::{TaskInfo, TestCases},
    handler::file_handler,
    parser::url_parser::{self, TaskUrl},
};
use itertools::Itertools;

pub struct FetchTestSuitesService {
    dao: Dao,
}

impl FetchTestSuitesService {
    pub fn new(dao: Dao) -> Self {
        Self { dao }
    }

    pub fn fetch_test_suites(&self, url: &str, config: &Config) -> Result<Vec<String>, Error> {
        let TasksInfo {
            url,
            contest_url,
            task_screen_names,
        } = self.fetch_tasks_info(url)?;

        let test_suites = self.dao.fetch_test_suites(&url)?;
        file_handler::save_test_suites(&test_suites, &config.file.test)?;

        let tasks_info = task_screen_names
            .into_iter()
            .zip(test_suites)
            .map(|(task_screen_name, TestCases { task, .. })| TaskInfo {
                task,
                contest_url: contest_url.clone(),
                task_screen_name,
            })
            .collect_vec();
        file_handler::save_tasks_info(&tasks_info, &config.file.tasks_info)?;

        let tasks = tasks_info
            .into_iter()
            .map(|task_info| task_info.task)
            .collect();
        Ok(tasks)
    }

    fn fetch_tasks_info(&self, url: &str) -> Result<TasksInfo, Error> {
        let tasks_info = match url.parse()? {
            TaskUrl::TasksPrint { url, contest_url } => {
                let tasks_url = format!("{contest_url}/tasks");
                let task_screen_names = self.dao.fetch_task_screen_names(&tasks_url)?;
                TasksInfo {
                    url,
                    contest_url,
                    task_screen_names,
                }
            }
            TaskUrl::Task {
                url,
                contest_url,
                task_screen_name,
            } => TasksInfo {
                url,
                contest_url,
                task_screen_names: vec![task_screen_name],
            },
        };

        Ok(tasks_info)
    }
}

#[derive(Debug)]
struct TasksInfo {
    url: String,
    contest_url: String,
    task_screen_names: Vec<String>,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Dao(#[from] dao::Error),

    #[error(transparent)]
    FileHandler(#[from] file_handler::Error),

    #[error(transparent)]
    InvalidUrl(#[from] url_parser::Error),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        config::{File, Url},
        handler::http_handler::HttpHandler,
    };
    use ureq::Agent;

    #[test]
    #[ignore]
    fn test_fetch_tasks_info() {
        // Setup
        let http_handler = HttpHandler::new(Agent::new());
        let dao = Dao::new(http_handler, "Dummy CSRF Token".to_string());
        let service = FetchTestSuitesService::new(dao);

        let url = "https://atcoder.jp/contests/abc388";

        // Run
        let tasks_info = service.fetch_tasks_info(url).unwrap();

        // Verify
        println!("{:#?}", tasks_info);
        assert_eq!(7, tasks_info.task_screen_names.len());
    }

    #[test]
    #[ignore]
    fn test_fetch_test_suites() {
        // Setup
        let config = Config {
            file: File {
                session_data: "".to_string(),
                tasks_info: "tests/data/tasks_info.toml".to_string(),
                test: "tests/data/test".to_string(),
            },
            url: Url {
                homepage: "".to_string(),
                login: "".to_string(),
            },
        };

        let http_handler = HttpHandler::new(Agent::new());
        let dao = Dao::new(http_handler, "Dummy CSRF Token".to_string());
        let service = FetchTestSuitesService::new(dao);

        let url = "https://atcoder.jp/contests/abc388";

        // Run
        let test_suites = service.fetch_test_suites(url, &config).unwrap();

        // Verify
        println!("{:#?}", test_suites);
        assert_eq!(7, test_suites.len());
    }
}
