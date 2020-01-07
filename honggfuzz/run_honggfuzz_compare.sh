#!/bin/sh
cargo update
HFUZZ_RUN_ARGS="-u -V -Q" cargo hfuzz run fuzz_target_compare