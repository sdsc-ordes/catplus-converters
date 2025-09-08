#!/usr/bin/env bash
set positional-arguments
set shell := ["bash", "-cue"]

root_dir := `git rev-parse --show-toplevel`
shapes_url := "https://github.com/sdsc-ordes/catplus-ontology/releases/download/v2.1.0/catplus_ontology.ttl"
# Default recipe to list all recipes.
default:
    just --list --no-aliases

# Build all crates
build *args:
    cargo build {{args}}

# Test all crates
test *args:
    cargo test {{args}}

alias fmt := format
# Format all crates
format *args:
    cargo fmt {{args}}

# Lint all code
lint *args:
  cargo clippy \
    --no-deps \
    -- -D warnings -A clippy::needless_return {{args}}

dev:
  just nix::develop

# Run the converter.
convert *args:
  cargo run --bin converter -- {{args}}

# Run the validation.
[group('validation')]
validate +args:
  cargo run \
    --bin validation \
    -- \
      --endpoint http://localhost:8001 \
      {{args}}

# Start validation server.
[group('validation')]
shacl-start:
  docker run \
    -d \
    --rm \
    --name catplus-shacl-api \
    -p 8001:8000 \
    -e SHAPES_URL={{shapes_url}} \
    ghcr.io/sdsc-ordes/shacl-api:develop

# Stop validation server.
[group('validation')]
shacl-stop:
  docker stop catplus-shacl-api &

# Manage container image
mod image './tools/just/image.just'
# Nix operations
mod nix './tools/just/nix.just'
