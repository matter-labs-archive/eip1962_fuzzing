#!/bin/sh
cargo install cargo-fuzz
cargo update
CC=clang CXX=clang++ NUM_JOBS=32 cargo +nightly fuzz run fuzz_target_compare_ops --jobs=32 -- -max_len=16000