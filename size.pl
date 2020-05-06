#!/usr/bin/perl
use Cwd;
use File::Basename;
my $base = basename(getcwd);

while ($_ = $ARGV[0]) {
    ($d,$i,$m,$l,$u,$g,$r,$s) = stat $_;
    if (/^\//) {
        printf "%s: %d\n", $_, $s;
    } else {
        printf "%s/%s: %d\n", $base, $_, $s;
    }
    shift;
}
