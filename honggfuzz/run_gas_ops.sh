#!/bin/sh
cargo install honggfuzz
cargo update
HFUZZ_RUN_ARGS="-u -V -n 32" cargo hfuzz run fuzz_target_gas