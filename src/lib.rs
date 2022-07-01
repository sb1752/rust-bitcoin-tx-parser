use buffer::ReadBuffer;
use clap::{App, Arg};
use hex::FromHex;
use sha2::{Digest, Sha256};
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
    inputs: Vec<Input>,
    outputs: Vec<Output>,
    locktime: u32,
    transaction_id: String,
}

#[derive(Debug)]
struct Input {
    txid: String,
    vout: u32,
    script_sig: String,
    sequence: String,
}

#[derive(Debug)]
struct Output {
    amount: u64,
    script_pub_key: String,
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

fn read_8_bytes(bytes_slice: &mut &[u8]) -> MyResult<u64> {
    let mut buffer = [0; 8];
    bytes_slice.read(&mut buffer)?;
    Ok(u64::from_le_bytes(buffer)) // little endian
}

fn read_transaction(bytes_slice: &mut &[u8]) -> MyResult<String> {
    let mut buffer = [0; 32];
    bytes_slice.read(&mut buffer)?;
    buffer.reverse(); // txids are formatted in big endian
    Ok(hex::encode(buffer))
}

fn read_compact_size(bytes_slice: &mut &[u8]) -> MyResult<u64> {
    let mut marker = [0; 1];
    bytes_slice.read(&mut marker)?;
    let num;

    if marker[0] < 0xFD {
        num = u8::from_le_bytes(marker) as u64;
    } else if marker[0] <= 0xFD {
        let mut buffer = [0; 2];
        bytes_slice.read(&mut buffer)?;
        num = u16::from_le_bytes(buffer) as u64;
    } else if marker[0] <= 0xFE {
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

fn read_script(bytes_slice: &mut &[u8]) -> MyResult<String> {
    let size = read_compact_size(bytes_slice)? as usize; // maximum 10000 bytes so usize is appropriate

    let mut buffer = Vec::with_capacity(size); // accepts usize type
    bytes_slice.read_buffer(&mut buffer)?;

    Ok(hex::encode(buffer))
}

fn read_inputs(bytes_slice: &mut &[u8], input_count: u64) -> MyResult<Vec<Input>> {
    let mut inputs = vec![];
    for _ in 0..input_count {
        let txid = read_transaction(bytes_slice)?;
        let vout = read_4_bytes(bytes_slice)?;
        let script_sig = read_script(bytes_slice)?;
        let sequence = format!("{:#x}", read_4_bytes(bytes_slice)?);

        inputs.push(Input {
            txid,
            vout,
            script_sig,
            sequence,
        })
    }
    Ok(inputs)
}

fn read_outputs(bytes_slice: &mut &[u8], output_count: u64) -> MyResult<Vec<Output>> {
    let mut outputs = vec![];

    for _ in 0..output_count {
        let amount = read_8_bytes(bytes_slice)?;
        let script_pub_key = read_script(bytes_slice)?;

        outputs.push(Output {
            amount,
            script_pub_key,
        })
    }

    Ok(outputs)
}

fn hash_raw_transaction(bytes: &[u8]) -> MyResult<String> {
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    let hash1 = hasher.finalize();

    let mut hasher = Sha256::new();
    hasher.update(hash1);
    let mut hash2 = hasher.finalize();

    hash2.reverse(); // displayed in big endian

    Ok(format!("{:x}", hash2))
}

pub fn run(raw_transaction: RawTransaction) -> MyResult<()> {
    let bytes = Vec::<u8>::from_hex(&raw_transaction.hex)?;
    let mut bytes_slice = &bytes[..];

    // version is 4 bytes
    let version = read_4_bytes(&mut bytes_slice)?;

    // read input count
    let input_count = read_compact_size(&mut bytes_slice)?;

    // read inputs
    let inputs = read_inputs(&mut bytes_slice, input_count)?;

    // read output count
    let output_count = read_compact_size(&mut bytes_slice)?;

    // read outputs
    let outputs = read_outputs(&mut bytes_slice, output_count)?;

    let locktime = read_4_bytes(&mut bytes_slice)?;

    let transaction_id = hash_raw_transaction(&bytes)?;

    let parsed_tx = ParsedTransaction {
        version,
        input_count,
        inputs,
        outputs,
        locktime,
        transaction_id,
    };

    println!("{:#?}", parsed_tx);

    Ok(())
}
