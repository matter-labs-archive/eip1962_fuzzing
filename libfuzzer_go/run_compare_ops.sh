#!/bin/sh
cargo install cargo-fuzz
cd fuzz
# cargo clean eth_pairings_go
cargo update
cd ..
# CC=clang CXX=clang++ cargo +nightly fuzz run fuzz_target_compare_ops -- -max_len=16000 --jobs=10000
CC=clang CXX=clang++ NUM_JOBS=10000 cargo +nightly fuzz run --release fuzz_target_compare_ops -- -max_len=16000
# CC=clang CXX=clang++ cargo +nightly fuzz run --release fuzz_target_compare_ops -- -max_len=16000 -ignore_crashes=1 -fork=32 
# -help=1