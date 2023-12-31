#!/bin/bash

# Build project using `instrument-coverage` flag
CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' cargo build

# Run tests, aditional scripts migth be needed for ignored tests or specific test
CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='cargo-test-%p-%m.profraw' cargo test

# Generate report
rm -rf ./coverage
grcov . --binary-path ./target/debug/deps/ -s . -t html --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o ./coverage/html

# Remove temp files
rm `find ./ | grep .profraw`