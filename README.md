# Fuzzers for EIP1962

This repo contains preset scripts for fuzzing of Rust implementation, fuzzing gas estimator (those are not too interesting), and differential testing between C++ implementation and Rust implementation. All scripts are Rust files, so C++ is wrapper into the thin layer.

## Implementations

- [Rust](https://github.com/matter-labs/eip1962)
- [C++](https://github.com/matter-labs/eip1962_cpp)

## Libfuzzer

Requires:
- C++17 compiler
- Nightly Rust
- Install using [manual](https://github.com/rust-fuzz/cargo-fuzz)

Usage:
- `cd fuzz`
- `bash run_fuzz_compare.sh`

## Honggfuzz

Requires:
- C++17 compiler
- Stable Rust
- Install using [manual](https://github.com/rust-fuzz/honggfuzz-rs)

Usage:
- `cd honggfuzz`
- `bash run_honggfuzz_compare.sh`

## Tuning

- Change number of threads for fuzzers in scripts
- Download initial corpus from [here](https://fuzz-inputs.fra1.digitaloceanspaces.com/input.zip) and follow the instructions of either [here](https://github.com/rust-fuzz/cargo-fuzz) or [here](https://github.com/rust-fuzz/honggfuzz-rs)

## Recommendations and updates

- Honggfuzz is recommended for speed
- Now Honggfuzz saves all results instead of unique ones. Due to differential testing being implemented as "panic on different results" fuzzer considered different reasons as one crash case.
- run `cargo update` before running to fetch latest copies from github

## Extra 

Folder `cross-tester` contains few examples how to specify inputs from fuzzer reports debug and compare implementations.
