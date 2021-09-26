#!/bin/bash

run_test() {
  echo "Running: tests/${1}.xq"
  ./target/release/xq-test-runner "tests/${1}.xq"
}

cargo build --release
run_test syntax
