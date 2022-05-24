use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn handles_silly_input() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("silly-echo")?;
    cmd.arg("silly-input");

    cmd.assert()
        .success()
        .stdout(predicate::str::is_match("no").unwrap());

    Ok(())
}

#[test]
fn handles_nice() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("silly-echo")?;
    cmd.arg("69");
    cmd.arg("hey there");

    cmd.assert()
        .failure()
        .code(predicate::eq(66))
        .stdout(predicate::str::is_match("hey there -- nice").unwrap());

    Ok(())
}

#[test]
fn handles_nice_without_second_arg() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("silly-echo")?;
    cmd.arg("69");

    cmd.assert()
        .failure()
        .code(predicate::eq(66))
        .stdout(predicate::str::is_match("ha ha nice").unwrap());

    Ok(())
}

#[test]
fn handles_no_args() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("silly-echo")?;

    cmd.assert()
        .failure()
        .code(predicate::eq(86))
        .stdout(predicate::str::is_match("You're not doing it right.").unwrap());

    Ok(())
}

#[test]
fn rejects_nums() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("silly-echo")?;
    cmd.arg("1");

    cmd.assert()
        .failure()
        .code(predicate::eq(1))
        .stdout(predicate::str::is_match("I don't wanna.").unwrap());

    Ok(())
}
