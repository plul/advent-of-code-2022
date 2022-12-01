_default:
	cargo check
	cargo test
	cargo clippy --tests -- -D rust-2018-idioms -D warnings
	cargo +nightly fmt --check
	cargo run

# List recipes
list: 
	@just --list

# Edit this justfile
edit:
	$EDITOR {{justfile()}}

# Format code
fmt:
	cargo +nightly fmt
