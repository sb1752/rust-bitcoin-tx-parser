use assert_cmd::Command;
use std::error::Error;
use std::fs;

type TestResult = Result<(), Box<dyn Error>>;

const PRG: &str = "rust_bitcoin_tx_parser";

#[test]
fn parse_transaction() -> TestResult {
    let input = fs::read_to_string("tests/transactions/tx1.txt")?;
    let expected = fs::read_to_string("tests/expected/tx1.txt")?;

    Command::cargo_bin(PRG)?
        .arg(input)
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}
