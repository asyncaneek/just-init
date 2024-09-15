use anyhow::Result;
use assert_cmd::prelude::*; // Add methods on commands
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
