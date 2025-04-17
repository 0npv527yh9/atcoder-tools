use crate::{
    dto::{
        config::{Config, LanguageConfig},
        Command, Diff, TestCase, TestCaseFile,
    },
    error::UnwrapOrExit,
    handler::{command_handler, file_handler, terminal_handler},
};
use std::{
    path::Path,
    process::{ExitStatus, Output},
};

pub fn run(
    config: &Config,
    language: String,
    task: String,
    test_case_filter: Option<Vec<String>>,
    verbose: bool,
) -> bool {
    let (language_config, test_dir) = setup(config, language).unwrap_or_exit();
    test(language_config, test_dir, task, test_case_filter, verbose).unwrap_or_exit()
}

fn setup(config: &Config, language: String) -> Result<(&LanguageConfig, &Path), Error> {
    let language_config = config
        .user_config
        .language_config(&language)
        .ok_or(Error::Config(language))?;
    let test_dir = &config.app_config.path.test;

    Ok((language_config, test_dir))
}

fn test(
    language_config: &LanguageConfig,
    test_dir: &Path,
    task: String,
    test_cases: Option<Vec<String>>,
    verbose: bool,
) -> Result<bool, Error> {
    if !compile(&language_config.compile)? {
        return Ok(false);
    }

    let test_case_files = file_handler::load_test_cases(test_dir, &task, test_cases)?;

    let diffs = verify(&language_config.execute, test_case_files)?;

    terminal_handler::print_diffs(&diffs, verbose)?;

    Ok(diffs.is_empty())
}

fn compile(command: &Option<Command>) -> Result<bool, Error> {
    let result = if let Some(command) = command {
        let exit_status = command_handler::run::<ExitStatus>(command, None)?;
        exit_status.success()
    } else {
        true
    };

    Ok(result)
}

fn verify(command: &Command, test_case_files: Vec<TestCaseFile>) -> Result<Vec<Diff>, Error> {
    let mut diffs = Vec::new();

    for test_case_file in test_case_files {
        let status = verify_one(command, test_case_file)?;
        match status {
            Status::RE(diff) => {
                diffs.push(diff);
                return Ok(diffs);
            }
            Status::WA(diff) => {
                diffs.push(diff);
            }
            Status::AC => (),
        }
    }

    Ok(diffs)
}

fn verify_one(command: &Command, test_case_file: TestCaseFile) -> Result<Status, Error> {
    let output = command_handler::run(command, Some(&test_case_file.test_case.input))?;
    judge(output, test_case_file)
}

fn judge(
    Output {
        status,
        stdout,
        stderr,
    }: Output,
    TestCaseFile {
        test_case: TestCase {
            input,
            output: expected,
        },
        file,
    }: TestCaseFile,
) -> Result<Status, Error> {
    let stdout = String::from_utf8(stdout)?;
    let stderr = String::from_utf8(stderr)?;

    let status = if expected.split_whitespace().eq(stdout.split_whitespace()) {
        Status::AC
    } else {
        let actual = format!("{stdout}\n\n{stderr}");

        let diff = Diff {
            input,
            expected,
            actual,
            file,
        };

        if status.success() {
            Status::WA(diff)
        } else {
            Status::RE(diff)
        }
    };

    Ok(status)
}

enum Status {
    AC,
    WA(Diff),
    RE(Diff),
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Config of {0} Not Found in Config.toml")]
    Config(String),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error(transparent)]
    FileHandler(#[from] file_handler::Error),

    #[error(transparent)]
    TerminalHanlder(#[from] terminal_handler::Error),
}

#[cfg(test)]
mod tests {
    use std::os::windows::process::ExitStatusExt;

    use super::*;

    #[test]
    fn test_judge_ac() {
        // Setup
        let expected = "Hello World!";
        let actual = " Hello\n World!  \n";

        let output = Output {
            status: ExitStatus::from_raw(0),
            stdout: actual.bytes().collect(),
            stderr: vec![],
        };

        // Run
        let test_case_file = TestCaseFile {
            test_case: TestCase {
                input: "input".to_string(),
                output: expected.to_string(),
            },
            file: "test.txt".to_string(),
        };

        // Verify
        match judge(output, test_case_file) {
            Ok(Status::AC) => (),
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_judge_wa() {
        // Setup
        let expected = "expected";
        let actual = "e";

        let output = Output {
            status: ExitStatus::from_raw(0),
            stdout: actual.bytes().collect(),
            stderr: vec![],
        };

        // Run
        let test_case_file = TestCaseFile {
            test_case: TestCase {
                input: "input".to_string(),
                output: expected.to_string(),
            },
            file: "test.txt".to_string(),
        };

        // Verify
        match judge(output, test_case_file) {
            Ok(Status::WA(Diff {
                input,
                expected,
                actual,
                file,
            })) => {
                assert_eq!(input, "input");
                assert_eq!(expected, "expected");
                assert_eq!(actual, "e");
                assert_eq!(file, "test.txt");
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_judge_re() {
        // Setup
        let expected = "expected";
        let actual = "e";
        let stderr = "error".to_string();

        let output = Output {
            status: ExitStatus::from_raw(1),
            stdout: actual.bytes().collect(),
            stderr: stderr.bytes().collect(),
        };

        // Run
        let test_case_file = TestCaseFile {
            test_case: TestCase {
                input: "input".to_string(),
                output: expected.to_string(),
            },
            file: "test.txt".to_string(),
        };

        // Verify
        match judge(output, test_case_file) {
            Ok(Status::RE(Diff {
                input,
                expected,
                actual,
                file,
            })) => {
                assert_eq!(input, "input");
                assert_eq!(expected, "expected");
                assert_eq!(actual, "e\n\nerror");
                assert_eq!(file, "test.txt");
            }
            _ => unreachable!(),
        }
    }
}
