use std::fs;
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn dies_no_args() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
}

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("Hello, world!")
        .assert()
        .success();
}

#[test]
fn single_arg() {
    let tests = vec![
        ("Hello there", "tests/testdata/expected/hello1.txt"),
    ];
    let mut cmd = Command::cargo_bin("echor").unwrap();
    for (input, expected_file) in tests {
        let expected = fs::read_to_string(expected_file).unwrap();
        cmd.arg(input)
            .assert()
            .stdout(predicate::str::contains(expected));
    }
}