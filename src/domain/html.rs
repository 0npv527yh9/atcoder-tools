use super::page_type::{self, Tasks};
use crate::dto::{TestCase, TestCases, TestSuite};
use itertools::Itertools;
use regex::Regex;
use scraper::{selectable::Selectable, ElementRef, Selector};
use std::{marker::PhantomData, ops::Deref};

pub struct Html<PageType>(scraper::Html, PhantomData<fn() -> PageType>);

impl<PageType> From<String> for Html<PageType> {
    fn from(html: String) -> Self {
        let html = scraper::Html::parse_document(&html);
        Self(html, PhantomData)
    }
}

impl<PageType> Deref for Html<PageType> {
    type Target = scraper::Html;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Html<page_type::Home> {
    pub fn csrf_token(&self) -> Option<String> {
        self.select_one("[name=csrf_token]")
            .and_then(|element| element.attr("value"))
            .map(Into::into)
    }

    pub fn title(&self) -> Option<String> {
        self.select_one("title").map(|element| element.inner_html())
    }

    pub fn has_sign_up_button(&self) -> bool {
        self.select_one("#navbar-collapse > .navbar-right > li:nth-child(2) > a")
            .map(|element| element.inner_html())
            .map_or(false, |name| name == "Sign Up")
    }
}

impl Html<page_type::Task> {
    pub fn test_suite(&self) -> TestSuite {
        self.parse_task_tags()
            .into_iter()
            .filter_map(|task_tag| {
                Some(TestCases {
                    task: task_tag.title()?,
                    test_cases: task_tag.test_cases(),
                })
            })
            .collect()
    }

    fn parse_task_tags(&self) -> Vec<TaskTag<'_>> {
        self.select_all("#task-statement")
            .into_iter()
            .filter_map(|task_statement_tag| {
                Some(TaskTag(ElementRef::wrap(task_statement_tag.parent()?)?))
            })
            .collect()
    }
}

#[derive(Debug)]
struct TaskTag<'a>(ElementRef<'a>);

struct TaskTitleTag<'a>(ElementRef<'a>);

#[derive(Debug)]
struct TestCaseTag<'a>(ElementRef<'a>);

impl<'a> TaskTag<'a> {
    fn title(&self) -> Option<String> {
        self.0.select_one("span.h2").map(TaskTitleTag)?.title()
    }

    fn test_case_tags(&self) -> Vec<TestCaseTag<'a>> {
        let pattern = Regex::new(r"^(入|出)力例").unwrap();

        let test_case_labels = self
            .0
            .select_all("h3")
            .into_iter()
            .filter(|h3| pattern.is_match(&h3.inner_html()));

        test_case_labels
            .filter_map(|label| ElementRef::wrap(label.parent()?))
            .filter_map(|parent| parent.select_one("pre"))
            .map(TestCaseTag)
            .collect()
    }

    fn test_cases(&self) -> Vec<TestCase> {
        self.test_case_tags()
            .iter()
            .map(TestCaseTag::test_case)
            .tuples::<(_, _)>()
            .map(|(input, output)| TestCase { input, output })
            .collect()
    }
}

impl TaskTitleTag<'_> {
    fn title(&self) -> Option<String> {
        self.0
            .inner_html()
            .split_whitespace()
            .next()
            .map(Into::into)
    }
}

impl TestCaseTag<'_> {
    fn test_case(&self) -> String {
        self.0.inner_html()
    }
}

impl Html<Tasks> {
    pub fn task_screen_names(&self) -> Vec<String> {
        let pattern = Regex::new(r"^/contests/[^/]+/tasks/([^/]+)$").unwrap();

        let submit_url_tags = self.select_all("table > tbody > tr > td:first-child > a");

        let submit_urls = submit_url_tags
            .into_iter()
            .filter_map(|a_tag| a_tag.attr("href"));

        submit_urls
            .filter_map(|url| Some(pattern.captures(url)?.get(1)?.as_str().to_string()))
            .collect()
    }
}

trait Select<'a> {
    fn select_one(self, selectors: &str) -> Option<ElementRef<'a>>;
    fn select_all(self, selectors: &str) -> Vec<ElementRef<'a>>;
}

impl<'a, T> Select<'a> for T
where
    T: Selectable<'a>,
{
    fn select_one(self, selectors: &str) -> Option<ElementRef<'a>> {
        let selector = Selector::parse(selectors).expect("Invalid Selector");
        let element = self.select(&selector).next();
        element
    }

    fn select_all(self, selectors: &str) -> Vec<ElementRef<'a>> {
        let selector = Selector::parse(selectors).expect("Invalid Selector");
        let elements = self.select(&selector).collect();
        elements
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    #[ignore]
    fn test_csrf_token() {
        // Setup
        let html = utils::test::load_homepage_html();

        let expected = rpassword::prompt_password("CSRF Token").unwrap();

        // Run
        let actual = html.csrf_token().expect("CSRF Token Not Found");

        // Verify
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_title() {
        // Setup
        let html = utils::test::load_homepage_html();
        let expected = "AtCoder";

        // Run
        let actual = html.title().expect("ERROR: <title> Not Found");

        // Verify
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_task_tags() {
        // Setup
        let html = utils::test::load_task_print_html();

        // Run
        let task_tags = html.parse_task_tags();

        // Verify
        println!("{:?}", task_tags);
        assert_eq!(7, task_tags.len());
    }

    #[test]
    fn test_parse_title() {
        // Setup
        let html = utils::test::load_task_print_html();
        let task_tags = html.parse_task_tags();

        // Run
        let titles = task_tags
            .iter()
            .filter_map(|task_tag| task_tag.title())
            .collect_vec();

        // Verify
        assert_eq!(vec!["A", "B", "C", "D", "E", "F", "G"], titles);
    }

    #[test]
    fn test_test_case_tags() {
        // Setup
        let html = utils::test::load_task_print_html();
        let task_tag = &html.parse_task_tags()[0];

        // Run
        let test_case_tags = task_tag.test_case_tags();

        // Verify
        println!("{test_case_tags:?}");
        assert_eq!(10, test_case_tags.len());
    }

    #[test]
    fn test_parse_test_suite() {
        // Setup
        let html = utils::test::load_task_print_html();

        // Run
        let test_cases = html.test_suite();

        // Verify
        println!("{test_cases:#?}");
        assert_eq!(7, test_cases.len());
    }

    #[test]
    fn test_task_screen_names() {
        // Setup
        let html = utils::test::load_tasks_html();

        // Run
        let task_screen_names = html.task_screen_names();

        // Verify
        println!("{task_screen_names:#?}");
        assert_eq!(7, task_screen_names.len());
    }
}
