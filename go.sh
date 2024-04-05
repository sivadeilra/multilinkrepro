#!/bin/bash

set -e

# place cargo target dir outside of wsl filesystem
export CARGO_TARGET_DIR=/tmp/multi-target

cargo build

clang main.cpp \
    -o main \
    -L $CARGO_TARGET_DIR/debug \
    -lblue \
    -lgreen \
    -lpthread

