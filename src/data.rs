use anyhow::{Context, Result};
use std::{collections::HashSet, path::Path};

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

impl Data {
    pub fn parse_key_val(s: &str) -> Result<(String, String)> {
        let pos = s
            .find('=')
            .ok_or_else(|| anyhow::anyhow!("invalid KEY=value: no `=` found in `{}`", s))?;
        Ok((s[..pos].to_string(), s[pos + 1..].to_string()))
    }

    fn check_unique_keys(pairs: &[(String, String)]) -> Result<()> {
        let mut keys = HashSet::new();
        for (key, _) in pairs {
            if !keys.insert(key) {
                return Err(anyhow::anyhow!("Duplicate key found: {}", key));
            }
        }
        Ok(())
    }
}

impl TryFrom<Vec<(String, String)>> for Data {
    type Error = anyhow::Error;
    fn try_from(pairs: Vec<(String, String)>) -> Result<Self, Self::Error> {
        Data::check_unique_keys(&pairs)?;
        let json_value = pairs
            .into_iter()
            .map(|(k, v)| (k, serde_json::Value::String(v)))
            .collect::<serde_json::Map<String, serde_json::Value>>();
        Ok(Data(serde_json::Value::Object(json_value)))
    }
}

#[cfg(test)]
mod tests {
    use assert_fs::prelude::{FileWriteStr, PathChild};

    use super::*;

    #[test]
    fn test_data_from_key_value_pairs() -> Result<()> {
        let pairs = vec![
            ("key1".to_string(), "value1".to_string()),
            ("key2".to_string(), "value2".to_string()),
            ("key3".to_string(), "value3".to_string()),
        ];

        let data = Data::try_from(pairs)?;

        if let serde_json::Value::Object(map) = &data.0 {
            assert_eq!(map.get("key1").and_then(|v| v.as_str()), Some("value1"));
            assert_eq!(map.get("key2").and_then(|v| v.as_str()), Some("value2"));
            assert_eq!(map.get("key3").and_then(|v| v.as_str()), Some("value3"));
            assert_eq!(map.len(), 3);
        } else {
            panic!("Expected Object, got something else");
        }

        Ok(())
    }

    #[test]
    fn test_data_from_empty_pairs() -> Result<()> {
        let pairs: Vec<(String, String)> = vec![];

        let data = Data::try_from(pairs)?;

        if let serde_json::Value::Object(map) = &data.0 {
            assert!(map.is_empty());
        } else {
            panic!("Expected Object, got something else");
        }

        Ok(())
    }

    #[test]
    fn test_data_from_duplicate_keys() {
        let pairs = vec![
            ("key1".to_string(), "value1".to_string()),
            ("key1".to_string(), "value2".to_string()),
        ];

        let result = Data::try_from(pairs);

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Duplicate key found: key1"));
    }
    #[test]
    fn test_data_from_json_file() -> Result<()> {
        let json_content = r#"
        {
            "project_name": "MyAwesomeProject",
            "version": "1.0.0",
            "author": "John Doe",
            "dependencies": ["serde", "tokio", "reqwest"]
        }
        "#;

        let temp_dir = assert_fs::TempDir::new()?;
        let file_path = temp_dir.child("test_data.json");
        file_path.write_str(json_content)?;
        let data = Data::try_from(file_path.path())?;

        if let serde_json::Value::Object(map) = &data.0 {
            assert_eq!(
                map.get("project_name").and_then(|v| v.as_str()),
                Some("MyAwesomeProject")
            );
            assert_eq!(map.get("version").and_then(|v| v.as_str()), Some("1.0.0"));
            assert_eq!(map.get("author").and_then(|v| v.as_str()), Some("John Doe"));
            assert_eq!(
                map.get("dependencies").and_then(|v| v.as_array()),
                Some(
                    serde_json::json!(["serde", "tokio", "reqwest"])
                        .as_array()
                        .expect("JSON array should be valid")
                )
            );
            assert_eq!(map.len(), 4);
        } else {
            panic!("Expected Object, got something else");
        }

        Ok(())
    }
}
