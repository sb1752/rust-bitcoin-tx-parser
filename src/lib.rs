use clap::{App, Arg};
use hex::FromHex;
use std::error::Error;
use std::io::Read;

#[derive(Debug)]
pub struct Config {
    raw_transaction: String,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
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

    Ok(Config {
        raw_transaction: matches.value_of("raw_transaction").unwrap().to_string(),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let bytes = Vec::<u8>::from_hex(config.raw_transaction)?;
    let mut byte_array = &bytes[..];

    // version is 4 bytes
    let mut version = [0; 4];
    byte_array.read(&mut version)?;
    // convert bytes to integer using little endian
    let version_num = u32::from_le_bytes(version);

    println!("The version is: {:?}", version_num);

    // input count is 1 byte
    let mut inputs = [0; 1];
    byte_array.read(&mut inputs)?;
    let inputs_num = u8::from_le_bytes(inputs);

    println!("The number of inputs is: {:?}", inputs_num);

    Ok(())
}
