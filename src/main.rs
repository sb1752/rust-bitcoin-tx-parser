fn main() {
    if let Err(e) = rust_bitcoin_tx_parser::get_args().and_then(rust_bitcoin_tx_parser::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
