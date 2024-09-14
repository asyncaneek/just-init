use std::path::PathBuf;

pub struct Source {
    dir: PathBuf,
    pattern: String,
}

impl Default for Source {
    fn default() -> Self {
        Source {
            dir: Default::default(),
            pattern: "**/*".to_string(),
        }
    }
}
