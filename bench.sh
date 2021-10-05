#!/bin/sh

make -C bench_C
cargo build --release

NUM=999999

cmd="time bench_C/bench-collatz-C $NUM"
echo "\n$cmd"; bash -c "$cmd"

cmd="time target/release/bench-collatz-fast $NUM"
echo "\n$cmd"; bash -c "$cmd"

cmd="time target/release/bench-collatz-safe $NUM"
echo "\n$cmd"; bash -c "$cmd"

cmd="time target/release/bench-collatz-biguint $NUM"
echo "\n$cmd"; bash -c "$cmd"
