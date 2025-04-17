#!/bin/bash
set -e

RUSTFLAGS="-C opt-level=z -C panic=abort" \
cargo build --release --target wasm32-unknown-unknown

wasm-opt -Oz -o public/out.wasm \
  target/wasm32-unknown-unknown/release/igl_nano.wasm

SIZE=$(stat -c%s public/out.wasm)
echo "Optimized WASM size: $SIZE bytes"