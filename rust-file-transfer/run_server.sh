#!/bin/sh
ADDR=127.0.0.1:8000

export RUST_LOG=info

./target/release/server $ADDR
