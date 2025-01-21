use crate::{
    domain::{html::Html, page_type},
    dto::SessionData,
};
use std::{fs, path::Path};
use ureq::Agent;

#[derive(Debug)]
pub enum Error {
    FetchFailed(String),
    InvalidHtml(String),
    WriteFailed(String),
    ReadFailed(String),
    InvalidSessionData(String),
}

pub fn load_homepage_html() -> Html<page_type::Home> {
    let file = "tests/data/homepage.html";
    let url = "https://atcoder.jp/home";
    load_html::<page_type::Home>(file, url)
}

pub fn load_contest_page_html() -> Html<page_type::ContestHome> {
    let file = "tests/data/contest_page.html";
    let url = "https://atcoder.jp/contests/abc386";
    load_html::<page_type::ContestHome>(file, url)
}

pub fn load_tasks_html() -> Html<page_type::Tasks> {
    let file = "tests/data/tasks.html";
    let url = "https://atcoder.jp/contests/abc386/tasks";
    load_html::<page_type::Tasks>(file, url)
}

pub fn load_task_print_html() -> Html<page_type::Task> {
    let file = "tests/data/task_print.html";
    let url = "https://atcoder.jp/contests/abc386/tasks_print";
    load_html::<page_type::Task>(file, url)
}

pub fn load_task_page_html() -> Html<page_type::Task> {
    let file = "tests/data/task_page.html";
    let url = "https://atcoder.jp/contests/abc386/tasks/abc386_a";
    load_html::<page_type::Task>(file, url)
}

fn load_html<PageType>(file: &str, url: &str) -> Html<PageType> {
    fs::read_to_string(file)
        .or_else(|_| fetch_html(url).and_then(|html| save(file, html)))
        .map(Into::into)
        .expect("Error: Fail to load or fetch HTML file")
}

fn fetch_html(url: &str) -> Result<String, Error> {
    println!("Fetching {url}");

    Agent::new()
        .get(url)
        .call()
        .map_err(|_| Error::FetchFailed(url.to_string()))
        .and_then(|response| match response.into_string() {
            Ok(html) => Ok(html.replace("\r", "")),
            Err(_) => Err(Error::InvalidHtml(url.to_string())),
        })
}

fn save(path: &str, contents: String) -> Result<String, Error> {
    if let Some(dir) = Path::new(path).parent() {
        fs::create_dir_all(dir).map_err(|_| Error::WriteFailed(path.to_string()))?;
    }

    match fs::write(path, &contents) {
        Ok(_) => Ok(contents),
        Err(_) => Err(Error::WriteFailed(path.to_string())),
    }
}

pub fn load_session_data() -> SessionData {
    let file = "tests/data/session_data.json";
    let url = "https://atcoder.jp/home";

    match fs::read_to_string(file) {
        Ok(session_data) => serde_json::from_str(&session_data)
            .map_err(|_| Error::InvalidSessionData(file.to_string())),
        Err(_) => {
            fetch_session_data(url).and_then(|session_data| {
                match serde_json::to_string_pretty(&session_data) {
                    Ok(session_data_json) => save(file, session_data_json).map(|_| session_data),
                    Err(_) => Err(Error::InvalidSessionData(file.to_string())),
                }
            })
        }
    }
    .expect("Error: Fail to load or fetch Session Data")
}

fn fetch_session_data(url: &str) -> Result<SessionData, Error> {
    let agent = Agent::new();

    println!("Fetching {url}");

    agent
        .get(url)
        .call()
        .map_err(|_| Error::FetchFailed(url.to_string()))?;

    let cookies = agent.cookie_store().iter_unexpired().cloned().collect();
    Ok(SessionData {
        cookies,
        csrf_token: "Dummy CSRF Token".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_load_html() {
        load_homepage_html();
    }
}
