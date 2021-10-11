#!/bin/bash

set -e

STORAGE="${1:-memory}"

run_test() {
  if [[ ${STORAGE} == "rocksdb" ]]; then
    PATH="$(mktemp -d)"
    echo "Running: tests/${1}.xq -- Database Path: ${PATH}"

    ./target/release/xq-test-runner -d "${PATH}" "tests/${1}.xq"
  else
    echo "Running: tests/${1}.xq"
    ./target/release/xq-test-runner "tests/${1}.xq"
  fi
}

cargo build --no-default-features --features ${STORAGE}-storage --release
run_test syntax
run_test asserts
