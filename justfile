# Check, test, lint, then run
default: _build _build-release
	cargo nextest run
	cargo clippy --tests -- -D rust-2018-idioms
	cargo +nightly fmt --check
	./target/release/advent-of-code-2022
	hyperfine --shell=none --warmup 5 './target/release/advent-of-code-2022'

# List recipes
list:
	@just --list

# Run and benchmark
run +ARGS: _build _build-release
	cargo run -- {{ARGS}}
	hyperfine --shell=none --warmup 5 './target/release/advent-of-code-2022 {{ARGS}}'

# Benchmark each problem individually
bench-each:
	just bench 1 1
	just bench 1 2
	just bench 2 1
	just bench 2 2
	just bench 3 1
	just bench 3 2
	just bench 4 1
	just bench 4 2
	just bench 5 1
	just bench 5 2
	just bench 6 1
	just bench 6 2
	just bench 7 1
	just bench 7 2
	just bench 8 1
	just bench 8 2
	just bench 9 1
	just bench 9 2

# Benchmark a given problem (or all together if none specified)
bench *ARGS: _build-release
	hyperfine --shell=none --warmup 5 './target/release/advent-of-code-2022 {{ARGS}}'

# Edit this justfile
edit:
	$EDITOR {{justfile()}}

# Format code
fmt:
	cargo +nightly fmt

# Open docs
doc:
	cargo doc --open

_build:
	cargo build

_build-release:
	cargo build --release