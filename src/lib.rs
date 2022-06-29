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
    input_count: u64,
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

fn read_4_bytes(bytes_slice: &mut &[u8]) -> MyResult<u32> {
    let mut buffer = [0; 4];
    bytes_slice.read(&mut buffer)?;
    Ok(u32::from_le_bytes(buffer)) // little endian
}

fn read_var_int(bytes_slice: &mut &[u8]) -> MyResult<u64> {
    let mut marker = [0; 1];
    bytes_slice.read(&mut marker)?;
    let num;
    let marker_hex = hex::encode(marker);

    if marker_hex < hex::encode("0xFD") {
        num = u8::from_le_bytes(marker) as u64
    } else if marker_hex == hex::encode("0xFD") {
        let mut buffer = [0; 2];
        bytes_slice.read(&mut buffer)?;
        num = u16::from_le_bytes(buffer) as u64;
    } else if marker_hex == hex::encode("0xFE") {
        let mut buffer = [0; 4];
        bytes_slice.read(&mut buffer)?;
        num = u32::from_le_bytes(buffer) as u64;
    } else {
        let mut buffer = [0; 8];
        bytes_slice.read(&mut buffer)?;
        num = u64::from_le_bytes(buffer);
    }

    Ok(num)
}

pub fn run(raw_transaction: RawTransaction) -> MyResult<()> {
    let bytes = Vec::<u8>::from_hex(raw_transaction.hex)?;
    let mut bytes_slice = &bytes[..];

    // version is 4 bytes
    let version = read_4_bytes(&mut bytes_slice)?;

    // input count is var int
    let input_count = read_var_int(&mut bytes_slice)?;

    let parsed_tx = ParsedTransaction {
        version,
        input_count,
    };

    println!("{:?}", parsed_tx);

    Ok(())
}
