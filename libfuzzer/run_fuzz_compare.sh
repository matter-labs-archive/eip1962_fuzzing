#!/bin/sh
CC=clang CXX=clang++ NUM_JOBS=12 cargo +nightly fuzz run --release fuzz_target_compare --jobs=12 -- -max_len=8192