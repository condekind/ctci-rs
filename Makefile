
.PHONY: build test run

# Path to the module updater utility
TOOLS=tools

build:
	cargo build

test:
	cargo test

run:
	cargo run