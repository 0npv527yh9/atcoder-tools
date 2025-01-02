use scraper::{error::SelectorErrorKind, selectable::Selectable, ElementRef, Selector};

pub trait HtmlParser<'a> {
    fn csrf_token(&'a self) -> Option<String>;
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

        let expected = std::env::var("CSRF_TOKEN")
            .expect("You should set the `CSRF_TOKEN` as an environment variable.");

        // Run
        let actual = (&Html::parse_document(&html))
            .csrf_token()
            .expect("ERROR: CSRF Token Not Found");

        // Verify
        assert_eq!(expected, actual);
    }
}
