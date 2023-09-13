use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn exits_with_no_args() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("kv-store")?;

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("the following required arguments were not provided"))
        .code(predicate::ne(0));

    Ok(())
}
