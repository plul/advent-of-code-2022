# Check, test, lint, then run
default:
	cargo check
	cargo test
	cargo clippy --tests -- -D rust-2018-idioms
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
	
# Open docs
doc:
	cargo doc --open
