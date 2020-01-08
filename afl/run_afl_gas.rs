#!/bin/sh
cargo afl build --release --bin fuzz_target_gas
cargo afl fuzz -i in_gas -o out_gas -M fuzzer01 target/release/fuzz_target_gas