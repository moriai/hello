#!/bin/bash

BIN=./target/release/hello
#cargo init --name hello --vcs none

build-measure () {
    echo RUSTFLAGS="$RUSTFLAGS"
    cargo build --release --quiet
    # otool -L $BIN
    ../size.pl $BIN
    strip $BIN
    ../size.pl $BIN
    cargo clean >/dev/null 2>&1
    echo
}

build-measure
