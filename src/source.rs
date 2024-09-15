use anyhow::{anyhow, Result};
use std::path::Path;
use std::path::PathBuf;

const TERA_GLOB_ALL_PATTERN: &str = "**/*";

#[derive(Debug, Clone)]
pub struct Source {
    pub dir: PathBuf,
    pub pattern: String,
}

impl Source {
    pub fn new(dir: &Path) -> Self {
        Self {
            dir: dir.to_path_buf(),
            pattern: TERA_GLOB_ALL_PATTERN.to_string(),
        }
    }

    pub fn use_pattern(mut self, pattern: &str) -> Self {
        self.pattern = pattern.to_string();
        self
    }
}

impl TryFrom<&Path> for Source {
    type Error = anyhow::Error;
    fn try_from(value: &Path) -> Result<Self> {
        if !value.exists() {
            return Err(anyhow!("could not read source `{}`", value.display()));
        }

        Ok(Source::new(value))
    }
}

impl Default for Source {
    fn default() -> Self {
        Source {
            dir: Default::default(),
            pattern: TERA_GLOB_ALL_PATTERN.to_string(),
        }
    }
}
