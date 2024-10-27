#!/bin/sh
GIT_DIR=$(cd $(dirname $0); pwd)

export RUST_BACKTRACE=1
cargo build --release


