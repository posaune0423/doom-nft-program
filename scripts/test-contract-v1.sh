#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

"$ROOT_DIR/scripts/build-test-sbf.sh"
BPF_OUT_DIR="$ROOT_DIR/target/test-sbf" cargo test -p tests --lib -- --test-threads=1
