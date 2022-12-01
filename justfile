_default:
	@just --list

# Run 
run:
	cargo run

# Check project
check: test lint fmt-check

# Run clippy
lint:
	cargo clippy --tests -- -D rust-2018-idioms -D warnings
	cargo +nightly fmt --check

# Run tests
test:
	cargo test

# Check whether code is correctly formatted
fmt-check:
	cargo +nightly fmt --check

# Format code
fmt:
	cargo +nightly fmt
