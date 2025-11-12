.PHONY: fmt clippy test build doc publish-dry-run all

fmt:
	cargo fmt --all

clippy:
	cargo clippy --all

test:
	cargo test

build:
	cargo build --release

doc:
	cargo doc --no-deps

publish-dry-run:
	cargo publish --dry-run

all: fmt clippy test build doc
