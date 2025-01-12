use scraper::{error::SelectorErrorKind, selectable::Selectable, ElementRef, Selector};

pub trait HtmlParser {
    fn csrf_token(self) -> Option<String>;
    fn title(self) -> Option<String>;
}

impl<'a, T> HtmlParser for T
where
    T: Select<'a>,
{
    fn csrf_token(self) -> Option<String> {
        self.select_one("[name=csrf_token]")
            .ok()
            .flatten()
            .and_then(|element| element.attr("value"))
            .map(Into::into)
    }

    fn title(self) -> Option<String> {
        self.select_one("title")
            .ok()
            .flatten()
            .map(|element| element.inner_html())
    }
}

trait Select<'a> {
    fn select_one(
        self,
        selectors: &'static str,
    ) -> Result<Option<ElementRef<'a>>, SelectorErrorKind<'static>>;

    fn select_all(
        self,
        selectors: &'static str,
    ) -> Result<Vec<ElementRef<'a>>, SelectorErrorKind<'static>>;
}

impl<'a, T> Select<'a> for T
where
    T: Selectable<'a>,
{
    fn select_one(
        self,
        selectors: &'static str,
    ) -> Result<Option<ElementRef<'a>>, SelectorErrorKind<'static>> {
        let selector = Selector::parse(selectors)?;
        let element = self.select(&selector).next();
        Ok(element)
    }

    fn select_all(
        self,
        selectors: &'static str,
    ) -> Result<Vec<ElementRef<'a>>, SelectorErrorKind<'static>> {
        let selector = Selector::parse(selectors)?;
        let elements = self.select(&selector).collect();
        Ok(elements)
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
        let actual = (&Html::parse_document(&html))
            .csrf_token()
            .expect("CSRF Token Not Found");

        // Verify
        assert_eq!(expected, actual);
    }

    #[test]
    #[ignore]
    fn test_title() {
        // Setup
        let html = utils::test::load_homepage_html();
        let expected = "AtCoder";

        // Run
        let actual = (&Html::parse_document(&html))
            .title()
            .expect("ERROR: <title> Not Found");

        // Verify
        assert_eq!(expected, actual);
    }
}
