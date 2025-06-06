use crate::domain::url::FetchTaskUrl;
use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Login
    Login {
        /// Check login status
        #[arg(short, long)]
        check: bool,
    },

    /// Fetch test suite
    #[command(name = "fetch-test", visible_alias = "f")]
    FetchTestSuite {
        /// URL of a contest page or a task page
        ///
        /// The following formats are supported:
        /// - Contest Page URL: https://atcoder.jp/contests/<contest>
        /// - Task Page URL: https://atcoder.jp/contests/<contest>/task/<task>
        #[arg(verbatim_doc_comment)]
        url: FetchTaskUrl,
    },

    /// Test
    #[command(visible_alias = "t")]
    Test {
        language: String,

        task: String,

        /// e.g. "--test-cases 1 3" specifies that test cases 1 and 3 will be used, and test case 2 will be skipped.
        /// If not specified, all test cases will be used.
        #[arg(verbatim_doc_comment, short, long, num_args = 1.., value_parser=append_txt_extension)]
        test_cases: Option<Vec<String>>,

        #[arg(long, short)]
        verbose: bool,
    },
}

fn append_txt_extension(s: &str) -> Result<String, String> {
    let file = if s.ends_with(".txt") {
        s.to_string()
    } else {
        format!("{s}.txt")
    };

    Ok(file)
}
