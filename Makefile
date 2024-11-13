build:
	cargo build

run:
	cargo run file ./data/equations.txt

test:
	cargo test

fmt:
	cargo fmt

clippy:
	cargo clippy

all: fmt clippy test