use anyhow::Result;
use assert_cmd::prelude::*; // Add methods on commands
use assert_fs::prelude::*; // Test with file
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn file_doesnt_exist() -> Result<()> {
    let mut cmd = Command::cargo_bin("just-init")?;

    let bad_file_path = "test/file/doesnt/exist";

    cmd.arg("--source")
        .arg(bad_file_path)
        .arg("--data")
        .arg(bad_file_path)
        .arg("--output")
        .arg(bad_file_path);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("could not read file"));

    Ok(())
}

#[test]
fn generate_file() -> Result<()> {
    const FILE_SOURCE_CONTENT: &str = r#"
{{namespace}}::{{class}}
"#;
    const FILE_DATA_CONTENT: &str = r#"
{
    "namespace": "just_namespace",
    "class": "just_class"
}
"#;

    let source = assert_fs::NamedTempFile::new("source.txt")?;
    source.write_str(FILE_SOURCE_CONTENT)?;

    let data = assert_fs::NamedTempFile::new("data.json")?;
    data.write_str(FILE_DATA_CONTENT)?;

    let output = assert_fs::TempDir::new()?;

    let mut cmd = Command::cargo_bin("just-init")?;

    cmd.arg("--source")
        .arg(source.path())
        .arg("--data")
        .arg(data.path())
        .arg("--output")
        .arg(output.path());

    cmd.assert().success();
    output.assert(predicate::path::exists());
    output.child("source.txt").assert(predicate::path::exists());
    output.close()?;

    Ok(())
}
