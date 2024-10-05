use anyhow::Result;
use ureq::Agent;

struct HttpHandler {
    agent: Agent,
}

impl HttpHandler {
    fn get(&self, url: &str) -> Result<String> {
        let response = self.agent.get(url).call()?;
        let html = response.into_string()?;
        Ok(html)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    #[ignore]
    fn test_get() {
        // Setup
        let url = std::env::var("URL")
            .expect("You should set the target `URL` as an environment variable.");
        let expected_file = std::env::var("EXPECTED_FILE")
            .expect("You should set the `EXPECTED_FILE` as an environment variable.");

        let expected = fs::read_to_string(expected_file).unwrap();
        let expected = expected.trim().split('\n').collect::<Vec<_>>();

        // Run
        let http_handler = HttpHandler {
            agent: Agent::new(),
        };
        let actual = http_handler.get(&url).unwrap();
        let actual = actual.replace("\r", "");
        let actual = actual.trim().split('\n').collect::<Vec<_>>();

        // Verify
        assert_eq!(expected.len(), actual.len());
        for (expected, actual) in expected.iter().zip(actual.iter()) {
            if !actual.contains("csrf_token") && !actual.contains("csrfToken") {
                assert_eq!(expected, actual);
            }
        }
    }
}
