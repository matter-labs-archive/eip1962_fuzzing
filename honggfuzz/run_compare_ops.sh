#!/bin/sh
cargo install honggfuzz
cargo update
NUM_JOBS=12 HFUZZ_RUN_ARGS="-u -n 32" cargo hfuzz run fuzz_target_compare_ops