#!/bin/sh
# cargo install --git https://github.com/rust-fuzz/honggfuzz-rs.git
cargo install honggfuzz
cargo update
# HFUZZ_RUN_ARGS="-u -V -n 32 -Q --exit_upon_crash" cargo hfuzz run fuzz_target_compare_ops
HFUZZ_RUN_ARGS="-u -V -n 32 --tmout_sigvtalrm --max_file_size 16000" cargo hfuzz run fuzz_target_compare_ops