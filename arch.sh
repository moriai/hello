#!/bin/sh

case ${arch:=`uname -m`} in
arm64|arm64e)   arch=aarch64 ;;
esac

echo $arch
