# Check, test, lint, then run
default: test lint fmt-check run

# Edit this justfile
edit:
    $EDITOR {{ justfile() }}

# List recipes
list:
    @just --list

# Run tests
test:
    cargo nextest run

# Clippy
lint:
    cargo clippy --tests -- -D rust-2018-idioms

# Run (debug) specified problem or all if unspecified
debug *ARGS:
    cargo run -- {{ ARGS }}

# Run (release) specified problem or all if unspecified
run *ARGS:
    cargo run --release -- {{ ARGS }}

# Benchmark a given problem (or all together if none specified)
bench *ARGS:
    cargo build --release
    hyperfine --shell=none --warmup 5 './target/release/advent-of-code-2022 {{ ARGS }}'

# Format code
fmt:
    cargo +nightly fmt
    just --unstable --fmt

fmt-check:
    cargo +nightly fmt --check
