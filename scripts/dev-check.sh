#!/usr/bin/env bash

set -euo pipefail

echo "[def] formatting..."
cargo fmt --all

echo "[dev] linting..."
cargo clippy --workspace --all-targets --all-features -q

echo "[dev] checking..."
cargo check --workspace

echo "[dev] ok."