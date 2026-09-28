#!/bin/bash

set -u

wsm="./target/wasm32-wasip1/release-wasi/path2ancestor.wasm"

run_wasi(){
  wasmtime \
    run \
    --env ENV_PATH="$1" \
    "${wsm}"
}

#run_wasi /Users/me/Documents/path/to/file.dat
run_wasi Documents/path/to/file.dat
run_wasi Downloads/test.iso
