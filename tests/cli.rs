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
        .arg("--data-file") // Updated argument
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

    const OUTPUT_DATA_CONTENT: &str = r#"
just_namespace::just_class
"#;

    let source_dir = assert_fs::TempDir::new()?;
    let source = source_dir.child("source.txt");
    source.write_str(FILE_SOURCE_CONTENT)?;

    let data = assert_fs::NamedTempFile::new("data.json")?;
    data.write_str(FILE_DATA_CONTENT)?;

    let output_dir = assert_fs::TempDir::new()?;

    source_dir.assert(predicate::path::exists());
    source.assert(predicate::path::exists());
    output_dir.assert(predicate::path::exists());

    let mut cmd = Command::cargo_bin("just-init")?;

    cmd.arg("--source")
        .arg(source_dir.path())
        .arg("--data-file")
        .arg(data.path())
        .arg("--output")
        .arg(output_dir.path());

    cmd.assert().success();
    let output_file = output_dir.child("source.txt");
    output_file.assert(predicate::path::exists());
    output_file.assert(predicates::ord::eq(OUTPUT_DATA_CONTENT));
    output_dir.close()?;

    Ok(())
}
