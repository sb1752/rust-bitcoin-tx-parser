use assert_cmd::Command;
use std::error::Error;
use std::fs;

type TestResult = Result<(), Box<dyn Error>>;

const PRG: &str = "rust_bitcoin_tx_parser";

fn parse_transaction(transaction: &str) -> TestResult {
    let input = fs::read_to_string(format!("tests/transactions/{}.txt", transaction))?;
    let expected = fs::read_to_string(format!("tests/expected/{}.txt", transaction))?;

    Command::cargo_bin(PRG)?
        .arg(input)
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn parse_tx1() -> TestResult {
    parse_transaction("tx1")
}

#[test]
fn parse_tx2() -> TestResult {
    parse_transaction("tx2")
}

#[test]
fn parse_tx3() -> TestResult {
    parse_transaction("tx3")
}
