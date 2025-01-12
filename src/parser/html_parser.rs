use scraper::{error::SelectorErrorKind, selectable::Selectable, ElementRef, Selector};

pub trait HtmlParser<'a> {
    fn csrf_token(&'a self) -> Option<String>;
    fn title(&'a self) -> Option<String>;
}

impl<'a, T> HtmlParser<'a> for T
where
    T: Selectable<'a> + Copy,
{
    fn csrf_token(&'a self) -> Option<String> {
        self.select_one("[name=csrf_token]")
            .ok()
            .flatten()
            .and_then(|e| e.attr("value"))
            .map(Into::into)
    }

    fn title(&'a self) -> Option<String> {
        self.select_one("title")
            .ok()
            .flatten()
            .map(|element| element.inner_html())
    }
}

trait Select<'a>: Selectable<'a> + Copy {
    fn select_one<'b>(
        &'a self,
        selectors: &'b str,
    ) -> Result<Option<ElementRef<'a>>, SelectorErrorKind<'b>> {
        let selector = Selector::parse(selectors)?;
        let element = self.select(&selector).next();
        Ok(element)
    }

    fn select_all<'b>(
        &'a self,
        selectors: &'b str,
    ) -> Result<Vec<ElementRef<'a>>, SelectorErrorKind<'b>> {
        let selector = Selector::parse(selectors)?;
        Ok(self.select(&selector).collect())
    }
}

impl<'a, T: Selectable<'a> + Copy> Select<'a> for T {}

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
