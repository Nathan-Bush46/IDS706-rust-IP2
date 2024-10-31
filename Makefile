release:
	cargo run --release
build:
	cargo build

lint:
	cargo clippy

run:
	cargo run

test: run
	cargo test
	
all: build test lint run