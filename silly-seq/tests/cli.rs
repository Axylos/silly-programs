use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn handles_empty() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("silly-seq")?;

    cmd.assert().failure().code(predicate::eq(2)).stdout(
        predicate::str::is_match("ouch.  these are not the droids you are looking for\n").unwrap(),
    );

    Ok(())
}

#[test]
fn handles_valid_input() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("silly-seq")?;
    cmd.arg("1");

    cmd.assert()
        .success()
        .stdout(predicate::str::is_match("In").unwrap());

    Ok(())
}

#[test]
fn handles_invalid_input() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("silly-seq")?;
    cmd.arg("a");

    cmd.assert()
        .failure()
        .code(predicate::eq(4))
        .stderr(predicate::str::is_match("maybe try a number").unwrap());

    Ok(())
}
