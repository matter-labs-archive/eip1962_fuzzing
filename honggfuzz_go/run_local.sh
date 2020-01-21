#!/bin/sh
# cargo install -f --git https://github.com/shamatar/honggfuzz-rs.git honggfuzz
cargo update
# HFUZZ_RUN_ARGS="-u -V -n 32 -Q --exit_upon_crash" cargo hfuzz run fuzz_target_compare_ops
HFUZZ_RUN_ARGS="-u -V -n 32" cargo hfuzz run fuzz_target_compare_ops