# Usage: make all | make fmt | make lint | make check

.PHONY: dev fmt check clippy run-cli run-repl

dev:
	./scripts/dev-check.sh

fmt:
	cargo fmt --all

check:
	cargo check --workspace

clippy:
	cargo clippy --workspace --all-targets --all-features -q

run-cli:
	cargo run -p cli

run-repl:
	cargo run -p repl
