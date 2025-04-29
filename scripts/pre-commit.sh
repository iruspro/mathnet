#!/bin/sh

set -e

echo "Running cargo fmt (mathnet-server)..."
cargo fmt --manifest-path mathnet-server/Cargo.toml --all -- --check

echo "Running cargo clippy (mathnet-server)..."
cargo clippy --manifest-path mathnet-server/Cargo.toml --all-targets --all-features -- -D warnings

# echo "Running cargo fmt (mathnet-client)..."
# cargo fmt --manifest-path client/Cargo.toml --all -- --check

# echo "Running cargo clippy (mathnet-client)..."
# cargo clippy --manifest-path client/Cargo.toml --all-targets --all-features -- -D warnings

echo "All checks passed."