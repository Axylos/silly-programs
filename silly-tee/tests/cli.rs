use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn handles_empty_input() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("silly-tee")?;

    cmd.assert()
        .failure()
        .code(predicate::eq(7))
        .stdout(predicate::str::is_match("whoopsie").unwrap());

    Ok(())
}

#[test]
fn handles_silly_input() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("silly-tee")?
        .write_stdin("hey there")
        .assert()
        .success()
        .stderr(predicate::str::is_match("hey there").unwrap())
        .stdout(predicate::str::is_match("104 101 121 32 116 104 101 114 101 \n").unwrap());

    Ok(())
}

#[test]
fn handles_answer() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("silly-tee")?
        .write_stdin("hey")
        .assert()
        .failure()
        .code(predicate::eq(42))
        .stderr(predicate::str::is_match("You've found the magic answer").unwrap())
        .stdout(predicate::str::is_match("\n\nFoLl0w 743 w4173 ra8817 n008\n").unwrap());

    Ok(())
}
