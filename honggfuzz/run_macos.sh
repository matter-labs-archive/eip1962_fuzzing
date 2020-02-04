#!/bin/sh
# cargo install --git https://github.com/shamatar/honggfuzz-rs.git honggfuzz
# cargo install -f --path ../../honggfuzz-rs honggfuzz
cargo update
# HFUZZ_RUN_ARGS="-u -V -n 12 --tmout_sigvtalrm --max_file_size 16000" cargo hfuzz run fuzz_target_compare_ops_macos
# HFUZZ_RUN_ARGS="-u -V -n 12 --tmout_sigvtalrm --max_file_size 16000" cargo hfuzz run fuzz_target_gas --no-default-features --features=macos
HFUZZ_RUN_ARGS="-u -V -n 12 --tmout_sigvtalrm --max_file_size 16000 -Q -v" cargo hfuzz run fuzz_target_compare_ops_macos --no-default-features --features=macos > log.log