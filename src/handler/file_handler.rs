use crate::dto::{TestCase, TestCases, TestSuite};
use serde::{de::DeserializeOwned, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn save_test_suite(test_suite: &TestSuite, test_dir: &Path) -> Result<(), Error> {
    for TestCases { task, test_cases } in test_suite {
        let task_test_dir = test_dir.join(task);
        let input_dir = task_test_dir.join("in");
        let output_dir = task_test_dir.join("out");

        fs::create_dir_all(&input_dir).with_path(&input_dir)?;
        fs::create_dir_all(&output_dir).with_path(&output_dir)?;

        for (i, TestCase { input, output }) in test_cases.iter().enumerate() {
            let file = format!("{}.txt", i + 1);

            let input_file = input_dir.join(&file);
            let output_file = output_dir.join(&file);

            fs::write(&input_file, input).with_path(&input_file)?;
            fs::write(&output_file, output).with_path(&output_file)?;
        }
    }
    Ok(())
}

pub fn save<T>(file_path: &Path, data: &T) -> Result<(), Error>
where
    T: Serialize,
{
    let contents = serde_json::to_string_pretty(data).with_path(file_path)?;
    fs::write(file_path, &contents).with_path(file_path)?;
    Ok(())
}

pub fn load<T>(file_path: &Path) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    let data = serde_json::from_str(&fs::read_to_string(file_path).with_path(file_path)?)
        .with_path(file_path)?;
    Ok(data)
}

pub fn load_toml<T>(file_path: &Path) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    let data = toml::from_str(&fs::read_to_string(file_path).with_path(file_path)?)
        .with_path(file_path)?;
    Ok(data)
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{source}: {path}")]
    IO {
        #[source]
        source: std::io::Error,
        path: PathBuf,
    },

    #[error("{message}: {path}")]
    Serde { message: String, path: PathBuf },
}

trait WithPath<T, E> {
    fn with_path(self, path: &Path) -> Result<T, E>;
}

impl<T> WithPath<T, Error> for Result<T, std::io::Error> {
    fn with_path(self, path: &Path) -> Result<T, Error> {
        self.map_err(|source| Error::IO {
            source,
            path: path.to_path_buf(),
        })
    }
}

impl<T> WithPath<T, Error> for Result<T, serde_json::Error> {
    fn with_path(self, path: &Path) -> Result<T, Error> {
        self.map_err(|source| Error::Serde {
            message: source.to_string(),
            path: path.to_path_buf(),
        })
    }
}

impl<T> WithPath<T, Error> for Result<T, toml::de::Error> {
    fn with_path(self, path: &Path) -> Result<T, Error> {
        self.map_err(|source| Error::Serde {
            message: source.to_string(),
            path: path.to_path_buf(),
        })
    }
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
        let result = save_test_suite(&test_suite, Path::new("tests/data/test"));

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
        let result = save(Path::new("tests/data/tasks_info.json"), &tasks_info);

        // Verify
        assert!(result.is_ok());
    }
}
