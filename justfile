# Check, test, lint, then run
default:
	cargo nextest run
	cargo clippy --tests -- -D rust-2018-idioms
	cargo +nightly fmt --check
	cargo run --release

# Edit this justfile
edit:
	$EDITOR {{justfile()}}

# List recipes
list:
	@just --list

# Run (Debug) specified problem or all if unspecified
run +ARGS:
	cargo run -- {{ARGS}}

# Benchmark a given problem (or all together if none specified)
bench *ARGS:
	cargo build --release
	hyperfine --shell=none --warmup 5 './target/release/advent-of-code-2022 {{ARGS}}'

# Format code
fmt:
	cargo +nightly fmt
