#!/bin/sh
cargo update
NUM_JOBS=12 HFUZZ_RUN_ARGS="-u -n 12" cargo hfuzz run fuzz_target_compare