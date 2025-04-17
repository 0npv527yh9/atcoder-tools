use crate::dto::Diff;
use itertools::Itertools;
use std::{io, iter};
use terminal_size::{Height, Width};

pub fn read_credentials() -> io::Result<Credentials> {
    let username = rprompt::prompt_reply("Username:")?;
    let password = rpassword::prompt_password("Password:")?;
    Ok(Credentials { username, password })
}

pub fn ask_for_retry() -> io::Result<bool> {
    let input = rprompt::prompt_reply("Retry? (y/[n]):")?.to_lowercase();
    Ok(&input == "y" || &input == "yes")
}

pub struct Credentials {
    pub username: String,
    pub password: String,
}

pub fn print_diffs(diffs: &[Diff], verbose: bool) -> Result<(), Error> {
    let (Width(w), Height(h)) = terminal_size::terminal_size().ok_or(Error::TerminalSize)?;

    for diff in diffs {
        let diff = make_diff(diff, verbose, (w as usize, h as usize));
        println!("{diff}");
    }

    Ok(())
}

fn make_diff(
    Diff {
        input,
        expected,
        actual,
        file,
    }: &Diff,
    verbose: bool,
    terminal_size: (usize, usize),
) -> String {
    let (max_width, max_height) = terminal_size;

    let mut lines = Vec::new();

    // Test case file name
    let title = make_title(file, "=", max_width);
    lines.push(title);

    for (title, content) in [("Input", input), ("Expected", expected), ("Actual", actual)] {
        let title = make_title(title, "-", max_width);
        lines.push(title);

        let content = content.trim_end();
        let content = if verbose {
            content.to_string()
        } else {
            trim(content, (max_width, max_height))
        };
        lines.push(content);
    }

    lines.join("\n")
}

fn make_title(title: &str, style: &str, max_width: usize) -> String {
    format!("{:^width$}", title, width = max_width).replace(' ', style)
}

fn trim(text: &str, (max_width, max_height): (usize, usize)) -> String {
    let lines = text.lines().collect_vec();

    let dot3 = "...";
    let left_width = (max_width - dot3.len()) >> 1;
    let right_width = (max_width - dot3.len()) - left_width;

    let height_trimmed_lines = if lines.len() <= max_height {
        lines
    } else {
        let half_height = (max_height - 1) >> 1;
        lines[..half_height]
            .iter()
            .copied()
            .chain(iter::once(dot3))
            .chain(lines[lines.len() - half_height..].iter().copied())
            .collect()
    };

    let trimmed_text = height_trimmed_lines
        .into_iter()
        .map(|line| {
            let line = line.trim_end();
            if line.len() <= max_width {
                line.to_string()
            } else {
                format!(
                    "{}{dot3}{}",
                    &line[..left_width],
                    &line[line.len() - right_width..]
                )
            }
        })
        .join("\n");

    trimmed_text
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to get terminal size")]
    TerminalSize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_title() {
        let title = make_title("1.txt", "=", 10);
        assert_eq!("==1.txt===", title)
    }

    #[test]
    fn test_make_diff() {
        let diff = Diff {
            input: "1 2\n3\n".to_string(),
            expected: "4 5\n6\n".to_string(),
            actual: "7 8\n9\n".to_string(),
            file: "test.txt".to_string(),
        };
        let verbose = true;

        let output = make_diff(&diff, verbose, (14, 14));
        assert_eq!(
            "\
===test.txt===
----Input-----
1 2
3
---Expected---
4 5
6
----Actual----
7 8
9",
            output
        );
    }

    #[test]
    fn test_make_diff_with_white_space() {
        let diff = Diff {
            input: "".to_string(),
            expected: " ".to_string(),
            actual: "\n".to_string(),
            file: "test.txt".to_string(),
        };
        let verbose = true;

        let output = make_diff(&diff, verbose, (14, 14));
        assert_eq!(
            "\
===test.txt===
----Input-----

---Expected---

----Actual----
",
            output
        );
    }

    #[test]
    fn test_make_diff_without_verbose() {
        let diff = Diff {
            input: "a bc def hjk\n".to_string(),
            expected: "l\n".to_string(),
            actual: "m n o p q    \n\n\nr\n".to_string(),
            file: "test.txt".to_string(),
        };
        let verbose = false;

        let output = make_diff(&diff, verbose, (10, 3));
        assert_eq!(
            "\
=test.txt=
--Input---
a b... hjk
-Expected-
l
--Actual--
m n o p q
...
r",
            output
        );
    }

    #[test]
    fn test_trim() {
        let text = "a b\nc\nd e\nf\n\ngh i jk  \n";
        let trimmed = trim(text, (6, 3));
        assert_eq!("a b\n...\ng...jk", trimmed);
    }
}
