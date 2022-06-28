# Bitcoin Transaction Parser

## Project:
Command line tool for parsing raw transactions

## Purpose:
Practice rust and bitcoin transaction fundamentals

## Overview:

Setup:
```
cargo build
```

Help menu:
```
cargo run -- -h
```

Parse raw transaction:
```
cargo run -- [raw transaction hex]
```

### Testing

Raw transactions are placed in the `tests/transactions` folder. The expected parsed responses are placed in the `tests/expected` folder. Can run tests to confirm that raw transactions are properly parsed with:
```
cargo test
```