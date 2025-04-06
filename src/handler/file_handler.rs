use crate::{
    domain::path::TaskTestPath,
    dto::{
        config::{AppConfig, Config},
        TestCase, TestCases, TestSuite,
    },
};
use serde::{de::DeserializeOwned, Serialize};
use std::{
    env,
    fs::{self},
    path::{Path, PathBuf},
};

pub fn save_test_suite(test_dir: &Path, test_suite: &TestSuite) -> Result<(), Error> {
    for TestCases { task, test_cases } in test_suite {
        let test_path = TaskTestPath::new(test_dir, task);
        let input_dir = test_path.input_dir();
        let output_dir = test_path.output_dir();

        fs::create_dir_all(&input_dir).with_path(&input_dir)?;
        fs::create_dir_all(&output_dir).with_path(&output_dir)?;

        for (i, TestCase { input, output }) in test_cases.iter().enumerate() {
            let file = format!("{}.txt", i + 1);

            let input_file = test_path.input_file(&file);
            let output_file = test_path.output_file(&file);

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

pub fn load_config() -> Result<Config, Error> {
    // Load config for app
    let app_config: AppConfig = toml::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/config.toml"
    )))
    .with_path(Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/config.toml"
    )))?;

    // Move to the root directory for app
    let metadata_path = find_in_ancestors(&app_config.path.metadata)
        .unwrap_or_else(|| panic!("Failed to {:?} in ancestors", app_config.path.metadata));
    let root_path = metadata_path.parent().unwrap();
    env::set_current_dir(root_path).with_path(root_path)?;

    // Load config for user
    let user_config = load_toml(&app_config.path.user_config)?;

    Ok(Config {
        app_config,
        user_config,
    })
}

fn find_in_ancestors(target: &Path) -> Option<PathBuf> {
    let current_dir = env::current_dir().expect("Failed to get current directory");

    current_dir
        .ancestors()
        .map(|p| p.join(target))
        .find(|path| path.is_dir())
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

pub trait WithPath<T, E> {
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
        let result = save_test_suite(Path::new("tests/data/test"), &test_suite);

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
