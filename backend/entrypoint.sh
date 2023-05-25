#!/bin/sh
set -e

echo "Container's IP address: `awk 'END{print $1}' /etc/hosts`"

cd backend

export CARGO_HOME=/backend/cargo
export RUSTUP_TOOLCHAIN=stable

cargo fetch --locked --target "x86_64-unknown-linux-gnu"
cargo build --frozen --release --all-features