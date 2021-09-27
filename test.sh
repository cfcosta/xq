#!/bin/bash

set -e

run_test() {
  PATH="$(mktemp -d)"
  echo "Running: tests/${1}.xq -- Database Path: ${PATH}"
  ./target/release/xq-test-runner -d "${PATH}" "tests/${1}.xq"
}

cargo build --release
run_test syntax
run_test asserts
