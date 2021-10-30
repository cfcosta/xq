#!/bin/bash

set -e

STORAGE="${1:-memory}"

run_test() {
  if [[ ${STORAGE} == "rocksdb" ]]; then
    DB_PATH="$(mktemp -d)"
    echo "Running: tests/${1}.xq -- Database Path: ${DB_PATH}"

    ./target/release/xq-test-runner -d "${DB_PATH}" "tests/${1}.xq"
  else
    echo "Running: tests/${1}.xq"
    ./target/release/xq-test-runner "tests/${1}.xq"
    echo
  fi
}

cargo build --no-default-features --features ${STORAGE}-storage --release
run_test syntax
run_test asserts
run_test stress
run_test null
