#!/bin/sh

set -e

echo "Running cargo fmt (mathnet-server)..."
cargo fmt --manifest-path mathnet-server/Cargo.toml --all --check

echo "Running cargo clippy (mathnet-server)..."
cargo clippy --manifest-path mathnet-server/Cargo.toml --all-targets --all-features -- -D warnings

echo "Running cargo fmt (mathnet-frontend)..."
cargo fmt --manifest-path mathnet-frontend/Cargo.toml --all --check

echo "Running cargo clippy (mathnet-frontend)..."
cargo clippy --manifest-path mathnet-frontend/Cargo.toml --all-targets --all-features -- -D warnings

echo "All checks passed."