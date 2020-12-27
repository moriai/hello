#!/bin/sh

case "$1" in
/*)
    BINARY=$(echo "$1" | sed "s,^"$CARGO_MANIFEST_DIR"/,,")
    HOSTDIR="$CARGO_MANIFEST_DIR/target"
    GUESTDIR="/target"
    ;;
\./*)
    HOSTDIR="$PWD"
    GUESTDIR=/$(basename "$HOSTDIR")
    BINARY=$(echo "$1" | sed "s,^\.,"$GUESTDIR",")
    ;;
*)
    BINARY="$1"
    DIR=$(echo "$1" | sed 's,/.*$,,')
    HOSTDIR="$PWD/$DIR"
    GUESTDIR="/$DIR"
    ;;
esac

shift

exec docker run -it -v "$HOSTDIR":"$GUESTDIR" --rm alpine:latest "$BINARY" "$@"
