.PHONY: install

install:
	cargo install --path . --example rainbow

build:
	cargo build --all --release
