.PHONY: all
all: fmt build test lint

.PHONY: fmt
fmt:
	@buf format -w proto
	@cargo fmt --all

.PHONY: check
check:
	@cargo check

.PHONY: build
build:
	@cargo build

.PHONY: test
test:
	@cargo test

.PHONY: itest
itest:
	@RUST_LOG=off cargo test -- --ignored

.PHONY: lint
lint:
	@buf lint proto
	@cargo clippy

.PHONY: clean
clean:
	@cargo clean

.PHONY: run
run:
	@cargo run

.PHONY: release
release:
	@cargo build --release
