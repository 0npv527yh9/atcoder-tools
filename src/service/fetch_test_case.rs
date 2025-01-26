use crate::{
    dao::{self, Dao},
    domain::{
        page_type,
        url::{self, FetchTaskUrl, Url},
    },
    dto::{config::Config, TaskInfo, TestCases},
    handler::file_handler,
};
use itertools::Itertools;

pub struct FetchTestSuiteService {
    dao: Dao,
}

impl FetchTestSuiteService {
    pub fn new(dao: Dao) -> Self {
        Self { dao }
    }

    pub fn fetch_test_suite(
        &self,
        config: &Config,
        task_url: FetchTaskUrl,
    ) -> Result<Vec<String>, Error> {
        let TasksInfo {
            url,
            contest_url,
            task_screen_names,
        } = self.fetch_tasks_info(task_url)?;

        let test_suite = self.dao.fetch_test_suite(&url)?;
        file_handler::save_test_suite(&test_suite, &config.file.test)?;

        let tasks_info = task_screen_names
            .into_iter()
            .zip(test_suite)
            .map(|(task_screen_name, TestCases { task, .. })| TaskInfo {
                task,
                contest_url: contest_url.clone(),
                task_screen_name,
            })
            .collect_vec();
        file_handler::save(&config.file.tasks_info, &tasks_info)?;

        let tasks = tasks_info
            .into_iter()
            .map(|task_info| task_info.task)
            .collect();
        Ok(tasks)
    }

    fn fetch_tasks_info(&self, task_url: FetchTaskUrl) -> Result<TasksInfo, Error> {
        let tasks_info = match task_url {
            FetchTaskUrl::Contest {
                contest_url,
                tasks_print_url,
                tasks_url,
            } => {
                let task_screen_names = self.dao.fetch_task_screen_names(&tasks_url)?;
                TasksInfo {
                    url: tasks_print_url,
                    contest_url,
                    task_screen_names,
                }
            }
            FetchTaskUrl::Task {
                task_url,
                contest_url,
                task_screen_name,
            } => TasksInfo {
                url: task_url,
                contest_url,
                task_screen_names: vec![task_screen_name],
            },
        };

        Ok(tasks_info)
    }
}

#[derive(Debug)]
struct TasksInfo {
    url: Url<page_type::Task>,
    contest_url: Url<page_type::ContestHome>,
    task_screen_names: Vec<String>,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Dao(#[from] dao::Error),

    #[error(transparent)]
    FileHandler(#[from] file_handler::Error),

    #[error(transparent)]
    InvalidUrl(#[from] url::Error),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        dto::config::{File, Url},
        handler::http_handler::HttpHandler,
    };
    use ureq::Agent;

    #[test]
    #[ignore]
    fn test_fetch_tasks_info() {
        // Setup
        let http_handler = HttpHandler::new(Agent::new());
        let dao = Dao::new(http_handler, "Dummy CSRF Token".to_string());
        let service = FetchTestSuiteService::new(dao);

        let task_url = "https://atcoder.jp/contests/abc388".parse().unwrap();

        // Run
        let tasks_info = service.fetch_tasks_info(task_url).unwrap();

        // Verify
        println!("{:#?}", tasks_info);
        assert_eq!(7, tasks_info.task_screen_names.len());
    }

    #[test]
    #[ignore]
    fn test_fetch_test_suite() {
        // Setup
        let config = Config {
            file: File {
                session_data: "".to_string(),
                tasks_info: "tests/data/tasks_info.toml".to_string(),
                test: "tests/data/test".to_string(),
            },
            url: Url {
                homepage: "".to_string().into(),
                login: "".to_string().into(),
            },
        };

        let http_handler = HttpHandler::new(Agent::new());
        let dao = Dao::new(http_handler, "Dummy CSRF Token".to_string());
        let service = FetchTestSuiteService::new(dao);

        let task_url = "https://atcoder.jp/contests/abc388".parse().unwrap();

        // Run
        let test_suite = service.fetch_test_suite(&config, task_url).unwrap();

        // Verify
        println!("{:#?}", test_suite);
        assert_eq!(7, test_suite.len());
    }
}
