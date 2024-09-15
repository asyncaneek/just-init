use anyhow::{Context, Result};
use std::path::Path;

#[derive(Default, Debug, Clone, serde::Serialize)]
pub struct Data(serde_json::Value);

impl TryFrom<&Path> for Data {
    type Error = anyhow::Error;
    fn try_from(value: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(value)
            .with_context(|| format!("could not read file `{}`", value.display()))?;
        let json_value = serde_json::from_str(&content)?;
        Ok(Data(json_value))
    }
}
