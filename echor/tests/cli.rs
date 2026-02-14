use std::fs;
use assert_cmd::cargo;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
    let mut cmd = cargo::cargo_bin_cmd!("echor");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}

#[test]
fn runs() -> TestResult {
    let mut cmd = cargo::cargo_bin_cmd!("echor");
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
    let mut cmd = cargo::cargo_bin_cmd!("echor");
    for (input, expected_file) in tests {
        let expected = fs::read_to_string(expected_file)?;
        cmd.arg(input)
            .assert()
            .stdout(predicate::str::contains(expected));
    }

    Ok(())
}