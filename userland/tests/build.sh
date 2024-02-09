#! /bin/bash

set -e

x86_64-unknown-circinus-gcc -static -no-pie -O0 -std=c2x -fno-exceptions test.c -o test

cp -v test ../build

pushd ../../
just run_file sh
popd