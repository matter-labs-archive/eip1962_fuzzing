#!/bin/sh
cargo install cargo-fuzz
cd fuzz
# cargo clean eth_pairings_go
cargo update
cd ..
# CC=clang CXX=clang++ cargo +nightly fuzz run fuzz_target_compare_ops -- -max_len=16000 
CC=clang CXX=clang++ cargo +nightly fuzz run --release fuzz_target_compare_ops -- -max_len=16000 -jobs=1000 -workers=32
# CC=clang CXX=clang++ cargo +nightly fuzz run --release fuzz_target_compare_ops -- -max_len=16000 -ignore_crashes=1 -fork=32 
# -help=1