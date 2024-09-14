use anyhow::{anyhow, Result};
use std::path::PathBuf;

const TERA_GLOB_ALL_PATTERN: &str = "**/*";

pub struct Source {
    dir: PathBuf,
    pattern: String,
}

impl Source {
    pub fn new(dir: PathBuf) -> Result<Self> {
        if !dir.exists() {
            return Err(anyhow!("directory not found"));
        }

        Ok(Self {
            dir,
            pattern: TERA_GLOB_ALL_PATTERN.to_string(),
        })
    }

    pub fn use_pattern(mut self, pattern: &str) -> Self {
        self.pattern = pattern.to_string();
        self
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_source() {
        let source = Source::default();
        assert!(source.pattern == *TERA_GLOB_ALL_PATTERN);
    }

    #[test]
    fn new_source() {
        let source = Source::new(PathBuf::from("."));
        assert!(source.is_ok_and(|source| source.pattern == *TERA_GLOB_ALL_PATTERN));
    }

    #[test]
    fn new_source_custom_pattern() {
        let source = Source::new(PathBuf::from("."));
        assert!(source.is_ok());

        let source = source.unwrap().use_pattern("*");

        assert!(source.pattern == "*");
    }
}
