use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct Output(pub PathBuf);

impl Output {
    pub fn as_path(&self) -> &Path {
        &self.0
    }
}

impl From<PathBuf> for Output {
    fn from(value: PathBuf) -> Self {
        Output(value)
    }
}
