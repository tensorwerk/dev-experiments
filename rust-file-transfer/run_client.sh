#!/bin/sh
ADDR=127.0.0.1:8000

export RUST_LOG=info
export CHUNK_SIZE=1024

if [ -z "$1" ]; then
    echo "A file argument is required"
else
    ./target/release/client $1 $ADDR
fi
