#!/bin/sh
cargo afl build --release --bin fuzz_target_compare
cargo afl fuzz -i in_compare -o out_compare -M fuzzer01 target/release/fuzz_target_compare