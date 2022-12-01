_default:
	@just --list

# Edit this justfile
edit:
	$EDITOR {{justfile()}}

# Just DO IT! (checks a few things)
doit: test lint fmt-check

# Run 
run:
	cargo run

# Run clippy
lint:
	cargo clippy --tests -- -D rust-2018-idioms -D warnings

# Run tests
test:
	cargo test

# Check whether code is correctly formatted
fmt-check:
	cargo +nightly fmt --check

# Format code
fmt:
	cargo +nightly fmt
