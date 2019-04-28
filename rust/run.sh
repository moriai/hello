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

export RUSTFLAGS='-C opt-level=3'; build-measure
export RUSTFLAGS='-C opt-level=s'; build-measure
export RUSTFLAGS='-C opt-level=z'; build-measure

export RUSTFLAGS='-C opt-level=3 -C prefer-dynamic'; build-measure
export RUSTFLAGS='-C opt-level=s -C prefer-dynamic'; build-measure
export RUSTFLAGS='-C opt-level=z -C prefer-dynamic'; build-measure
