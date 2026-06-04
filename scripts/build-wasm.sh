#!/usr/bin/env sh
set -eu

ROOT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)"

cd "$ROOT_DIR/crates/sudoku-wasm"
RUSTFLAGS='--cfg getrandom_backend="wasm_js"' \
  wasm-pack build --target web --out-dir "$ROOT_DIR/apps/web/app/wasm"

mkdir -p "$ROOT_DIR/apps/web/public/wasm"
cp "$ROOT_DIR/apps/web/app/wasm/sudoku_wasm.js" "$ROOT_DIR/apps/web/public/wasm/"
cp "$ROOT_DIR/apps/web/app/wasm/sudoku_wasm_bg.wasm" "$ROOT_DIR/apps/web/public/wasm/"
