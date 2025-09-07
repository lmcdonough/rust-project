# Usage: make all | make fmt | make lint | make check

.PHONY: all fmt lint check dev

all: dev

fmt:
	cargo fmt --all

lint:
	cargo clippy --workspace --all-targets --all-features -q

check:
	cargo check --workspace

dev:
	./scripts/dev-check.sh
