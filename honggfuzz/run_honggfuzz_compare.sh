#!/bin/sh
cargo update
NUM_JOBS=12 HFUZZ_RUN_ARGS="-u -V -Q" cargo hfuzz run fuzz_target_compare