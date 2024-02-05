#! /bin/bash

set -e

mkdir -pv bdir
pushd bdir

cmake .. -G Ninja -DCMAKE_BUILD_TYPE=Debug && ninja install

pushd ../../
just run_file sh true
popd

popd