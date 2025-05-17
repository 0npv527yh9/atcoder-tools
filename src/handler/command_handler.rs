use crate::dto::Command;
use std::{
    io::Write,
    process::{self, Stdio},
};

pub fn run<T: ReturnType>(
    Command {
        command,
        args,
        working_dir,
    }: &Command,
    input: Option<&str>,
) -> Result<T, std::io::Error> {
    let mut command = process::Command::new(command);

    // Args
    command.args(args);

    // Working directory
    if let Some(working_dir) = working_dir {
        command.current_dir(working_dir);
    }

    // Stdin
    if input.is_some() {
        command.stdin(Stdio::piped());
    }

    let result = T::from_process(command, input)?;
    Ok(result)
}

pub trait ReturnType {
    fn from_process(command: process::Command, input: Option<&str>) -> Result<Self, std::io::Error>
    where
        Self: Sized;
}

impl ReturnType for process::Output {
    fn from_process(
        mut command: process::Command,
        input: Option<&str>,
    ) -> Result<Self, std::io::Error>
    where
        Self: Sized,
    {
        let mut child = command
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        if let Some(input) = input {
            if let Some(stdin) = &mut child.stdin {
                stdin.write_all(input.as_bytes())?;
            }
        }

        child.wait_with_output()
    }
}

impl ReturnType for process::ExitStatus {
    fn from_process(
        mut command: process::Command,
        input: Option<&str>,
    ) -> Result<Self, std::io::Error>
    where
        Self: Sized,
    {
        let mut child = command.stdout(Stdio::inherit()).spawn()?;

        if let Some(input) = input {
            if let Some(stdin) = &mut child.stdin {
                stdin.write_all(input.as_bytes())?;
            }
        }

        child.wait()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{path::PathBuf, str::FromStr};

    #[test]
    fn test_run() {
        let command = Command {
            command: "echo".to_string(),
            args: vec!["Hello, World!".to_string()],
            working_dir: None,
        };

        let result = run::<process::Output>(&command, None);
        let output = result.unwrap();
        assert_eq!(output.stdout, b"Hello, World!\n");
    }

    #[test]
    fn test_run_with_working_dir() {
        let command = Command {
            command: "ls".to_string(),
            args: vec![],
            working_dir: Some(PathBuf::from_str("src").unwrap()),
        };

        let result = run::<process::Output>(&command, None);
        let output = String::from_utf8(result.unwrap().stdout).unwrap();
        assert!(output.split_whitespace().any(|file| file == "main.rs"));
    }
}
