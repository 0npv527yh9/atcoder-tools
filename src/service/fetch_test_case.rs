use crate::{
    config::Config,
    dao::{self, Dao},
    dto::{TaskInfo, TestSuite},
    handler::file_handler,
    parser::url_parser::{self, TaskUrl},
};
use itertools::Itertools;

pub struct FetchTestSuitesService {
    dao: Dao,
}

impl FetchTestSuitesService {
    pub fn fetch_test_suites(&self, url: String, config: Config) -> Result<(), Error> {
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
            .map(|(task_screen_name, TestSuite { task, .. })| TaskInfo {
                task,
                contest_url: contest_url.clone(),
                task_screen_name,
            })
            .collect_vec();
        file_handler::save_tasks_info(&tasks_info, &config.file.tasks_info)?;

        Ok(())
    }

    fn fetch_tasks_info(&self, url: String) -> Result<TasksInfo, Error> {
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
