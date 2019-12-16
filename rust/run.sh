#!/bin/bash

. ./build-measure.sh

export RUSTFLAGS='-C opt-level=3'; build-measure
export RUSTFLAGS='-C opt-level=s'; build-measure
export RUSTFLAGS='-C opt-level=z'; build-measure

export RUSTFLAGS='-C opt-level=3 -C prefer-dynamic'; build-measure
export RUSTFLAGS='-C opt-level=s -C prefer-dynamic'; build-measure
export RUSTFLAGS='-C opt-level=z -C prefer-dynamic'; build-measure
