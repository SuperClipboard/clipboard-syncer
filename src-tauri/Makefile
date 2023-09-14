.PHONY: fmt clippy clean build pack all ci

all: clean fmt clippy

ci: fmt clippy

fmt:
	cargo fmt --all --

clippy:
	cargo clippy --  -D warnings

clean:
	rm -rf ./target
