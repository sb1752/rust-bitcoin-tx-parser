use clap::{App, Arg};
use hex::FromHex;
use std::error::Error;
use std::io::Read;

#[derive(Debug)]
pub struct RawTransaction {
    hex: String,
}

#[derive(Debug)]
#[allow(dead_code)] // remove warnings that fields are never read
struct ParsedTransaction {
    version: u32,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<RawTransaction> {
    let matches = App::new("Bitcoin Transaction Parser")
        .version("0.1.0")
        .author("Shaan Batra")
        .about("Bitcoin raw transaction parser")
        .arg(
            Arg::with_name("raw_transaction")
                .value_name("raw_transaction")
                .help("Raw transaction hex")
                .required(true),
        )
        .get_matches();

    Ok(RawTransaction {
        hex: matches.value_of("raw_transaction").unwrap().to_string(),
    })
}

pub fn run(raw_transaction: RawTransaction) -> MyResult<()> {
    let bytes = Vec::<u8>::from_hex(raw_transaction.hex)?;
    let mut byte_slice = &bytes[..];

    // version is 4 bytes
    let mut buffer = [0; 4];
    byte_slice.read(&mut buffer)?;
    // convert bytes to integer using little endian
    let version = u32::from_le_bytes(buffer);

    let parsed_tx = ParsedTransaction { version };

    println!("{:?}", parsed_tx);

    Ok(())
}
