#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="$ROOT_DIR/target/test-sbf"
MPL_CORE_PROGRAM_ID="CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d"

mkdir -p "$OUT_DIR"

if [[ ! -f "$OUT_DIR/mpl_core_program.so" ]]; then
  solana program dump -u m "$MPL_CORE_PROGRAM_ID" "$OUT_DIR/mpl_core_program.so"
fi
