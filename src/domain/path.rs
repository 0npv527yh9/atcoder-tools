use crate::handler::file_handler::{Error, WithPath};
use itertools::Itertools;
use std::{
    ffi::OsString,
    fs::{self, DirEntry},
    path::{Path, PathBuf},
};

pub struct TaskTestPath {
    path: PathBuf,
}

impl TaskTestPath {
    pub fn new(test_dir: &Path, task: &str) -> Self {
        Self {
            path: test_dir.join(task),
        }
    }

    pub fn input_dir(&self) -> PathBuf {
        self.path.join("in")
    }

    pub fn output_dir(&self) -> PathBuf {
        self.path.join("out")
    }

    pub fn input_file(&self, file: impl AsRef<Path>) -> PathBuf {
        self.input_dir().join(file)
    }

    pub fn output_file(&self, file: impl AsRef<Path>) -> PathBuf {
        self.output_dir().join(file)
    }

    pub fn list_files(&self) -> Result<Vec<OsString>, Error> {
        let input_dir = self.input_dir();

        let entries: Vec<DirEntry> = fs::read_dir(&input_dir)
            .with_path(&input_dir)?
            .map(|entry| entry.with_path(&input_dir))
            .collect::<Result<_, _>>()?;

        Ok(entries
            .into_iter()
            .filter(is_file)
            .map(|entry| entry.file_name())
            .sorted()
            .collect())
    }
}

fn is_file(entry: &DirEntry) -> bool {
    entry.file_type().map_or(false, |file| file.is_file())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_list_files() {
        // Setup
        let task_test_path = TaskTestPath::new(Path::new("tests/data/test"), "A");

        // Run
        let files = task_test_path.list_files().unwrap();

        // Verify
        assert_eq!(vec!["1.txt", "2.txt", "3.txt"], files);
    }
}
