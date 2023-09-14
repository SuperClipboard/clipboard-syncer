.PHONY: fmt clippy clean build pack all ci

all: clean fmt clippy test

ci: fmt clippy test

fmt:
	cargo fmt --all --

clippy:
	cargo clippy --  -D warnings

clean:
	rm -rf ./target

test:
	cargo test
