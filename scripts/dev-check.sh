#!/usr/bin/env bash

set -euo pipefail

echo "[def] formatting..."
cargo fmt --all

echo "[def] type-checking..."
cargo check --workspace

echo "[def] linting..."

cargo clippy --workspace --all-targets --all-features -q

echo "[def] smoke-run cli..."
cargo run -p cli

echo "[def] smoke-run repl..."
cargo run -p repl

echo "[ok] dev-check complete."
