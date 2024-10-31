release:
	cargo run --release
build:
	cargo build
test:
	cargo test

lint:
	cargo clippy

run:
	cargo run

all: build test lint run