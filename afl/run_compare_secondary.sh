#!/bin/sh
cargo afl fuzz -i in_compare -o out_compare -S fuzzer02 target/release/fuzz_target_compare
cargo afl fuzz -i in_compare -o out_compare -S fuzzer03 target/release/fuzz_target_compare
cargo afl fuzz -i in_compare -o out_compare -S fuzzer04 target/release/fuzz_target_compare
cargo afl fuzz -i in_compare -o out_compare -S fuzzer06 target/release/fuzz_target_compare
cargo afl fuzz -i in_compare -o out_compare -S fuzzer07 target/release/fuzz_target_compare
cargo afl fuzz -i in_compare -o out_compare -S fuzzer08 target/release/fuzz_target_compare