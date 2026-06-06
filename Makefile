test:
	cargo test --workspace
	cargo clippy --workspace --all-targets -- -D warnings

format:
	cargo fmt