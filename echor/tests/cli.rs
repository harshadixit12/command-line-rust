use std::fs;
use assert_cmd::Command;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}

#[test]
fn runs() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("Hello, world!")
        .assert()
        .success();

    Ok(())
}

#[test]
fn single_arg() -> TestResult {
    let tests = vec![
        ("Hello there", "tests/testdata/expected/hello1.txt"),
    ];
    let mut cmd = Command::cargo_bin("echor")?;
    for (input, expected_file) in tests {
        let expected = fs::read_to_string(expected_file)?;
        cmd.arg(input)
            .assert()
            .stdout(predicate::str::contains(expected));
    }

    Ok(())
}