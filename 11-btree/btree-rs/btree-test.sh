#!/usr/bin/env bash

if [ -z "$1" ]
then
	LEVEL=warn
else
	LEVEL=$1
fi
export RUST_BACKTRACE=full
export RUST_TEST_THREADS=1
export RUST_LOG=btree=$LEVEL

cargo test -- --nocapture

