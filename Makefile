.PHONY: build test sim clean docs

build:
	cargo build --workspace

test:
	cargo test --workspace

sim:
	cargo run -p codesim-desktop

docs:
	@echo "See docs/ for architecture and SDK documentation"

clean:
	cargo clean
