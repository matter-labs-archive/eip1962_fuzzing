#!/bin/sh
# cargo install --git https://github.com/shamatar/honggfuzz-rs.git honggfuzz
# cargo install -f --path ../../honggfuzz-rs honggfuzz
cargo update
# -Q -v
# HFUZZ_RUN_ARGS="-u -V -n 12 --tmout_sigvtalrm --max_file_size 16000" cargo hfuzz run fuzz_target_compare_ops_macos
# HFUZZ_RUN_ARGS="-u -V -n 12 --tmout_sigvtalrm --max_file_size 16000" cargo hfuzz run fuzz_target_gas --no-default-features --features=macos
# HFUZZ_RUN_ARGS="-u -V -n 12 --tmout_sigvtalrm --max_file_size 16000" cargo hfuzz run fuzz_target_compare_ops_macos --no-default-features --features=macos
HFUZZ_RUN_ARGS="-u -V -n 12 --tmout_sigvtalrm --max_file_size 16000" cargo hfuzz run-debug fuzz_target_compare_ops_macos SIGABRT.EXC_CRASH.PC.00007fff6eccb49a.STACK.000000011edb025c.ADDR.0000000000000000.TIME.2020-02-04.15.51.26.PID.20410.fuzz # --no-default-features --features=macos