use crate::dto::{TestCase, TestCases, TestSuite};
use serde::{de::DeserializeOwned, Serialize};
use std::fs;

pub fn save_test_suite(test_suite: &TestSuite, test_dir: &str) -> Result<(), Error> {
    for TestCases { task, test_cases } in test_suite {
        let input_dir = format!("{test_dir}/{task}/in");
        let output_dir = format!("{test_dir}/{task}/out");

        fs::create_dir_all(&input_dir)?;
        fs::create_dir_all(&output_dir)?;

        for (i, TestCase { input, output }) in test_cases.iter().enumerate() {
            let file = format!("{}.txt", i + 1);

            let input_file = format!("{input_dir}/{file}");
            let output_file = format!("{output_dir}/{file}");

            fs::write(&input_file, input)?;
            fs::write(&output_file, output)?;
        }
    }
    Ok(())
}

pub fn save<T>(file_path: &str, data: &T) -> Result<(), Error>
where
    T: Serialize,
{
    let contents = serde_json::to_string_pretty(data)?;
    fs::write(file_path, &contents)?;
    Ok(())
}

pub fn load<T>(file_path: &str) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    let data = serde_json::from_str(&fs::read_to_string(file_path)?)?;
    Ok(data)
}

pub fn load_toml<T>(file_path: &str) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    let data = toml::from_str(&fs::read_to_string(file_path)?).unwrap();
    Ok(data)
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
}

#[cfg(test)]
mod tests {
    use crate::dto::TaskInfo;

    use super::*;

    #[test]
    fn test_save_test_suite() {
        // Setup
        let test_suite = vec![
            TestCases {
                task: "A".to_string(),
                test_cases: vec![TestCase {
                    input: "1\n2\n".to_string(),
                    output: "3\n4\n".to_string(),
                }],
            },
            TestCases {
                task: "B".to_string(),
                test_cases: vec![
                    TestCase {
                        input: "1\n2\n".to_string(),
                        output: "3\n4\n".to_string(),
                    },
                    TestCase {
                        input: "1\n2\n".to_string(),
                        output: "3\n4\n".to_string(),
                    },
                ],
            },
        ];

        // Run
        let result = save_test_suite(&test_suite, "tests/data/test");

        // Verify
        assert!(result.is_ok());
    }

    #[test]
    fn test_save_data() {
        // Setup
        let tasks_info = [TaskInfo {
            task: "some-task".to_string(),
            contest_url: "contest-url".to_string().into(),
            task_screen_name: "some-contest_some-task".to_string(),
        }];

        // Run
        let result = save("tests/data/tasks_info.json", &tasks_info);

        // Verify
        assert!(result.is_ok());
    }
}
