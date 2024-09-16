use anyhow::Context;
use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct Output(pub PathBuf);

impl Output {
    pub fn as_path(&self) -> &Path {
        &self.0
    }
}

impl TryFrom<&Path> for Output {
    type Error = anyhow::Error;
    fn try_from(value: &Path) -> Result<Self> {
        if !value.exists() {
            fs::create_dir_all(value).with_context(|| {
                format!("failed to create output directory at {}", value.display())
            })?;
        }

        Ok(Output(value.to_path_buf()))
    }
}
