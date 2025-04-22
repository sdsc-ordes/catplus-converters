#!/usr/bin/env bash
set positional-arguments
set shell := ["bash", "-cue"]

root_dir := `git rev-parse --show-toplevel`
shapes_url := "https://github.com/sdsc-ordes/catplus-ontology/releases/latest/download/catplus_ontology.ttl"

# Default recipe to list all recipes.
default:
    just --list --no-aliases

# Build all crates
build *args:
    cargo build

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
# Run the converter.
run input_type input_file output_file *args:
    cd "{{root_dir}}/src/converter" && \
    cargo run --bin converter \
        "{{input_type}}" \
        "{{root_dir}}/{{input_file}}" \
        "{{root_dir}}/{{output_file}}" {{args}}

# Upload converter image.
upload-image:
   skopeo login ghcr.io
   skopeo copy \
    "docker-archive:.output/package/catplus-converter-image" \
    "docker://ghcr.io/sdsc-ordes/catplus-converter:latest"

# Build the catplus-converter Nix package.
nix-package *args:
    nix build ./tools/nix#catplus-converter \
        --out-link .output/build/bin/catplus-converter

# Build the catplus-converter-image Nix Docker image.
nix-image *args:
    nix build ./tools/nix#catplus-converter-image \
        --out-link .output/package/catplus-converter-image

# Enter a nix interpreter with loaded flake.
nix-repl:
    nix repl ./tools/nix

alias dev := nix-develop
# Enter a Nix development shell.
nix-develop *args:
    @echo "Starting nix developer shell in './tools/nix/flake.nix'."
    cmd=("$@") && \
    { [ -n "${cmd:-}" ] || cmd=("zsh"); } && \
    nix develop ./tools/nix#default --accept-flake-config --command "${cmd[@]}"

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
    -d -it --rm \
    --name catplus-shacl-api \
    -p 8001:8000 \
    -e SHAPES_URL={{shapes_url}} \
    ghcr.io/sdsc-ordes/shacl-api:develop

# Stop validation server.
[group('validation')]
shacl-stop:
  docker stop catplus-shacl-api &

