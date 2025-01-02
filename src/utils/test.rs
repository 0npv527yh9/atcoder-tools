use std::{fs, path::Path};

use ureq::Agent;

#[derive(Debug)]
pub enum Error {
    FetchFailed(String),
    InvalidHtml(String),
    WriteFailed(String),
}

pub fn load_homepage_html() -> String {
    let file = "tests/data/homepage.html";
    let url = "https://atcoder.jp/home";
    load_html(file, url)
}

pub fn load_contest_page_html() -> String {
    let file = "tests/data/contest_page.html";
    let url = "https://atcoder.jp/contests/abc386";
    load_html(file, url)
}

pub fn load_task_page_html() -> String {
    let file = "tests/data/task_page.html";
    let url = "https://atcoder.jp/contests/abc386/tasks/abc386_a";
    load_html(file, url)
}

fn load_html(file: &str, url: &str) -> String {
    fs::read_to_string(file)
        .or_else(|_| fetch_html(url).and_then(|html| save(file, html)))
        .expect("Error: Fail to load or fetch HTML file")
}

fn fetch_html(url: &str) -> Result<String, Error> {
    println!("Fetching {url}");

    Agent::new()
        .get(url)
        .call()
        .map_err(|_| Error::FetchFailed(url.to_string()))
        .and_then(|response| {
            response
                .into_string()
                .map_err(|_| Error::InvalidHtml(url.to_string()))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_load_html() {
        load_homepage_html();
    }
}
