#!/usr/bin/perl

while ($_ = $ARGV[0]) {
    ($d,$i,$m,$l,$u,$g,$r,$s) = stat $_;
    printf "%s: %d\n", $_, $s;
    shift;
}
