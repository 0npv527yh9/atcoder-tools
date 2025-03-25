use crate::{
    dao::{self, Dao},
    domain::{
        page_type::ContestHome,
        url::{self, FetchTaskUrl, Url},
    },
    dto::{config::Config, SessionData, TaskInfo},
    error::UnwrapOrExit,
    handler::{file_handler, http_handler::HttpHandler},
};
use itertools::Itertools;

pub fn run(config: &Config, task_url: FetchTaskUrl) {
    let dao = setup(config);
    fetch(config, &dao, task_url).unwrap_or_exit();
}

fn setup(config: &Config) -> Dao {
    let SessionData {
        cookies,
        csrf_token,
    } = file_handler::load(&config.file.session_data).unwrap_or_exit();

    let http_handler = HttpHandler::with_cookies(cookies);
    Dao::new(http_handler, csrf_token)
}

fn fetch(config: &Config, dao: &Dao, task_url: FetchTaskUrl) -> Result<(), Error> {
    let test_suite = dao.fetch_test_suite(task_url.task_url())?;
    file_handler::save_test_suite(&test_suite, &config.file.test)?;

    let task_names = test_suite
        .into_iter()
        .map(|test_cases| test_cases.task)
        .collect_vec();

    println!("Saved: {task_names:?}");

    let task_screen_names = fetch_task_screen_names(dao, &task_url)?;
    let task_info = create_task_info(task_names, task_screen_names, task_url.contest_url());
    file_handler::save(&config.file.tasks_info, &task_info)?;

    Ok(())
}

fn fetch_task_screen_names(dao: &Dao, task_url: &FetchTaskUrl) -> Result<Vec<String>, Error> {
    let task_screen_names = match task_url {
        FetchTaskUrl::Contest { tasks_url, .. } => dao.fetch_task_screen_names(tasks_url)?,
        FetchTaskUrl::Task {
            task_screen_name, ..
        } => vec![task_screen_name.clone()],
    };

    Ok(task_screen_names)
}

fn create_task_info(
    task_names: Vec<String>,
    task_screen_names: Vec<String>,
    contest_url: &Url<ContestHome>,
) -> Vec<TaskInfo> {
    task_names
        .into_iter()
        .zip(task_screen_names)
        .map(|(task, task_screen_name)| TaskInfo {
            task,
            task_screen_name,
            contest_url: contest_url.clone(),
        })
        .collect()
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
    use crate::handler::http_handler::HttpHandler;
    use ureq::Agent;

    #[test]
    #[ignore]
    fn test_fetch_task_screen_names() {
        // Setup
        let http_handler = HttpHandler::new(Agent::new());
        let dao = Dao::new(http_handler, "Dummy CSRF Token".to_string());

        let task_url = "https://atcoder.jp/contests/abc388".parse().unwrap();

        // Run
        let tasks_info = fetch_task_screen_names(&dao, &task_url).unwrap();

        // Verify
        println!("{:#?}", tasks_info);
        assert_eq!(7, tasks_info.len());
    }
}
