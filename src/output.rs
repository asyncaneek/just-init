use std::path::PathBuf;

pub struct Output {
    dir: PathBuf,
}

impl Default for Output {
    fn default() -> Self {
        Output {
            dir: PathBuf::from("."),
        }
    }
}
