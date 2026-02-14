use std::error::Error;
use randstr::randstr;
use assert_cmd::cargo;

#[test]
fn non_existent_file() -> Result<(), Box<dyn Error>> {
    const FILE_NAME_LENGTH: usize = 10;
    let mut randstrgen = randstr().len(FILE_NAME_LENGTH).letter().build();
    let random_file_name = randstrgen.generate();

    let expected = format!(r"Failed to open {}: No such file or directory \(os error 2\)", random_file_name);

    cargo::cargo_bin_cmd!("catr")
        .arg(&random_file_name)
        .assert()
        .success()
        .stderr(predicates::str::is_match(expected)?);

    Ok(())
}