use crate::dto::{TaskInfo, TestCase, TestSuite};
use std::fs;

pub fn save_test_suites(test_suites: &[TestSuite], test_dir: &str) -> Result<(), Error> {
    for TestSuite { task, test_cases } in test_suites {
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

pub fn save_tasks_info(tasks_info: &[TaskInfo], file_path: &str) -> Result<(), Error> {
    let contents = serde_json::to_string_pretty(tasks_info)?;
    fs::write(file_path, &contents)?;
    Ok(())
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
    use super::*;

    #[test]
    fn test_save_test_suites() {
        // Setup
        let test_suites = vec![
            TestSuite {
                task: "A".to_string(),
                test_cases: vec![TestCase {
                    input: "1\n2\n".to_string(),
                    output: "3\n4\n".to_string(),
                }],
            },
            TestSuite {
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
        let result = save_test_suites(&test_suites, "tests/data/test");

        // Verify
        assert!(result.is_ok());
    }

    #[test]
    fn test_save_tasks_info() {
        // Setup
        let tasks_info = [TaskInfo {
            task: "some-task".to_string(),
            contest_url: "contest-url".to_string(),
            task_screen_name: "some-contest_some-task".to_string(),
        }];

        // Run
        let result = save_tasks_info(&tasks_info, "tests/data/tasks_info.json");

        // Verify
        assert!(result.is_ok());
    }
}
