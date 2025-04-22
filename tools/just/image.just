set positional-arguments
set shell := ["bash", "-cue"]
root_dir := `git rev-parse --show-toplevel`

# Default recipe to list all recipes.
[private]
default:
  just --list image --no-aliases


# Build the catplus-converters-image Nix OCI image.
build *args:
    @echo "Building image derivation"
    just nix build
    nix build {{root_dir}}/tools/nix#catplus-converters-image \
        --out-link {{root_dir}}/build/image/catplus-converters-image

# Upload converters image.
push: build
   @echo "🐋 Uploading docker image"
   skopeo copy \
    "docker-archive:{{root_dir}}/build/image/catplus-converters-image" \
    "docker://ghcr.io/sdsc-ordes/catplus-converters:latest"
