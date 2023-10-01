_list:
    @just --list --unsorted

# Check project
check:
    just --unstable --fmt --check
    nix fmt -- --check .
    taplo fmt --check `fd --extension=toml`
    prettier --check `fd --extension=md`
    cargo fmt -- --check 
    cargo clippy --tests --examples -- -D warnings
    taplo lint `fd --extension=toml`
    RUSTDOCFLAGS='-Dwarnings' cargo doc --no-deps
    cargo nextest run
    nix flake show

# Format code
fmt:
    just --unstable --fmt
    nix fmt
    taplo fmt `fd --extension=toml`
    cargo fmt
    prettier --write `fd --extension=md`

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

# List nightly features in use
list-nightly-features:
    rg '^#!\[feature(.*)\]'
