use std::path::PathBuf;

const TERA_GLOB_ALL_PATTERN: &str = "**/*";

#[derive(Debug, Clone)]
pub struct Source {
    pub dir: PathBuf,
    pub pattern: String,
}

impl Source {
    pub fn new(dir: PathBuf) -> Self {
        Self {
            dir,
            pattern: TERA_GLOB_ALL_PATTERN.to_string(),
        }
    }

    pub fn use_pattern(mut self, pattern: &str) -> Self {
        self.pattern = pattern.to_string();
        self
    }
}

impl From<PathBuf> for Source {
    fn from(value: PathBuf) -> Self {
        Source::new(value)
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
