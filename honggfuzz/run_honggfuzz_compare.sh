#!/bin/sh
cargo update
NUM_JOBS=12 HFUZZ_RUN_ARGS="-u -v" cargo hfuzz run fuzz_target_compare