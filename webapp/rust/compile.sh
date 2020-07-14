#!/bin/sh -x

cd "$(dirname "$0")"
cargo build --release
mv ./target/release/isucari .
