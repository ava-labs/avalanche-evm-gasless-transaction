#!/usr/bin/env bash
set -xue

if ! [[ "$0" =~ scripts/tests.unit.sh ]]; then
  echo "must be run from repository root"
  exit 255
fi

RUST_LOG=debug cargo test --all \
-- --show-output

echo "ALL SUCCESS!"
