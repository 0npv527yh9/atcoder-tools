use std::path::{Path, PathBuf};

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
}
