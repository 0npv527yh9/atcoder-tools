use crate::dto::{TestCase, TestSuite};
use itertools::Itertools;
use scraper::{selectable::Selectable, ElementRef, Html, Selector};

pub trait HtmlParser {
    fn csrf_token(&self) -> Option<String>;
    fn title(&self) -> Option<String>;
    fn test_suites(&self) -> Vec<TestSuite>;
}

impl HtmlParser for Html {
    fn csrf_token(&self) -> Option<String> {
        self.select_one("[name=csrf_token]")
            .and_then(|element| element.attr("value"))
            .map(Into::into)
    }

    fn title(&self) -> Option<String> {
        self.select_one("title").map(|element| element.inner_html())
    }

    fn test_suites(&self) -> Vec<TestSuite> {
        let task_tags = parse_task_elements(self);
        task_tags
            .into_iter()
            .filter_map(|task_tag| {
                let title = task_tag.title_tag()?.title()?;
                let test_cases = task_tag
                    .test_case_tags()
                    .iter()
                    .map(TestCaseTag::test_case)
                    .tuples()
                    .map(|(input, output)| TestCase { input, output })
                    .collect();
                Some(TestSuite {
                    task: title,
                    test_cases,
                })
            })
            .collect()
    }
}

fn parse_task_elements(html: &Html) -> Vec<TaskTag<'_>> {
    html.select_all("span.h2")
        .into_iter()
        .map(TitleTag)
        .filter_map(Into::into)
        .collect()
}

#[derive(Debug)]
struct TaskTag<'a>(ElementRef<'a>);

struct TitleTag<'a>(ElementRef<'a>);

#[derive(Debug)]
struct TestCaseTag<'a>(ElementRef<'a>);

impl<'a> From<TitleTag<'a>> for Option<TaskTag<'a>> {
    fn from(tag: TitleTag<'a>) -> Self {
        Some(TaskTag(ElementRef::wrap(tag.0.parent()?)?))
    }
}

impl<'a> TaskTag<'a> {
    fn title_tag(&self) -> Option<TitleTag<'a>> {
        self.0.select_one("span.h2").map(TitleTag)
    }

    fn test_case_tags(&self) -> Vec<TestCaseTag<'a>> {
        let test_case_labels = self
            .0
            .select_all("h3")
            .into_iter()
            .filter(|h3| h3.inner_html().starts_with("Sample "));

        test_case_labels
            .filter_map(|label| ElementRef::wrap(label.parent()?))
            .filter_map(|parent| parent.select_one("pre"))
            .map(TestCaseTag)
            .collect()
    }
}

impl TitleTag<'_> {
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
    use scraper::Html;

    #[test]
    #[ignore]
    fn test_csrf_token() {
        // Setup
        let html = utils::test::load_homepage_html();

        let expected = rpassword::prompt_password("CSRF Token").unwrap();

        // Run
        let actual = Html::parse_document(&html)
            .csrf_token()
            .expect("CSRF Token Not Found");

        // Verify
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_title() {
        // Setup
        let html = utils::test::load_homepage_html();
        let expected = "AtCoder";

        // Run
        let actual = Html::parse_document(&html)
            .title()
            .expect("ERROR: <title> Not Found");

        // Verify
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_task_elements() {
        // Setup
        let html = utils::test::load_task_print_html();
        let html = Html::parse_document(&html);

        // Run
        let task_tags = parse_task_elements(&html);

        // Verify
        println!("{:?}", task_tags);
        assert_eq!(7, task_tags.len());
    }

    #[test]
    fn test_parse_title() {
        // Setup
        let html = utils::test::load_task_print_html();
        let html = Html::parse_document(&html);
        let task_tags = parse_task_elements(&html);

        // Run
        let titles = task_tags
            .iter()
            .filter_map(|task_tag| task_tag.title_tag()?.title())
            .collect_vec();

        // Verify
        assert_eq!(vec!["A", "B", "C", "D", "E", "F", "G"], titles);
    }

    #[test]
    fn test_test_case_tags() {
        // Setup
        let html = utils::test::load_task_print_html();
        let html = Html::parse_document(&html);
        let task_tag = &parse_task_elements(&html)[0];

        // Run
        let test_case_tags = task_tag.test_case_tags();

        // Verify
        println!("{test_case_tags:?}");
        assert_eq!(10, test_case_tags.len());
    }

    #[test]
    fn test_parse_test_suites() {
        // Setup
        let html = utils::test::load_task_print_html();
        let html = Html::parse_document(&html);

        // Run
        let test_cases = html.test_suites();

        // Verify
        println!("{test_cases:#?}");
        assert_eq!(7, test_cases.len());
    }
}
