#!/bin/sh
cargo install honggfuzz
cargo update
HFUZZ_RUN_ARGS="-u -V -Q --exit_upon_crash" cargo hfuzz run fuzz_target_compare_ops