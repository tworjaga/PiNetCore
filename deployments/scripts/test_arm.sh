#!/bin/bash
# ARM cross-compile test

rustup target add aarch64-unknown-linux-gnu
cd core-engine
cargo build --release --target aarch64-unknown-linux-gnu
echo "ARM build OK"
docker buildx build --platform linux/arm64 -t pinetcore-arm64 -f ../deployments/docker/Dockerfile.core ../core-engine/
echo "ARM Docker OK"

