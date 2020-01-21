#!/bin/sh
scp root@165.22.69.114:/root/eip1962_fuzzing/honggfuzz/hfuzz_workspace/fuzz_target_compare_ops/HONGGFUZZ.REPORT.TXT ./fuzz_vectors/
scp root@165.22.69.114:/root/eip1962_fuzzing/honggfuzz/hfuzz_workspace/fuzz_target_compare_ops/*.fuzz ./fuzz_vectors/