use std::{fmt::Display, process};

pub trait UnwrapOrExit<T> {
    fn unwrap_or_exit(self) -> T;
}

impl<T, E> UnwrapOrExit<T> for Result<T, E>
where
    E: Display,
{
    fn unwrap_or_exit(self) -> T {
        self.unwrap_or_else(|error| {
            eprintln!("{error}");
            process::exit(1)
        })
    }
}

pub trait ExpectOrExit<T> {
    fn expect_or_exit(self, msg: &str) -> T;
}

impl<T, E> ExpectOrExit<T> for Result<T, E> {
    fn expect_or_exit(self, msg: &str) -> T {
        self.unwrap_or_else(|_| {
            eprintln!("{msg}");
            process::exit(1)
        })
    }
}
