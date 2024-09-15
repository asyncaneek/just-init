use std::{collections::HashMap, path::PathBuf};

#[derive(Default, Debug, Clone, serde::Serialize)]
pub struct Data(HashMap<String, String>);

impl From<PathBuf> for Data {
    fn from(_value: PathBuf) -> Self {
        // let content = std::fs::read_to_string(value)?;
        // let json_value = serde_json::from_str(&content)?;
        Data::default()
    }
}
