use std::process::ExitCode;

fn main() -> ExitCode {
    let cli = atcoder_tools::parse_cli();
    match atcoder_tools::run(cli) {
        Ok(atcoder_tools::RunOutcome::Success) => ExitCode::SUCCESS,
        Ok(atcoder_tools::RunOutcome::Failure) => ExitCode::FAILURE,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
