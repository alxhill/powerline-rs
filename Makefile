.PHONY: install

install:
	cargo install --path . --no-default-features --features=bare-shell,libgit --example rainbow
