#!/bin/bash
set -e

echo "PiNetCore Installation for Raspberry Pi OS"

# Update system
sudo apt update &amp;&amp; sudo apt upgrade -y

# Install dependencies
sudo apt install -y build-essential pkg-config libssl-dev nftables sqlite3 wireguard

# Rust for ARM
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env
rustup target add aarch64-unknown-linux-gnu

# Clone project
git clone https://github.com/tworjaga/PiNetCore.git /opt/pinetcore
cd /opt/pinetcore

# Build core
cd core-engine
cargo build --release --target aarch64-unknown-linux-gnu

# Setup systemd
sudo cp deployments/systemd/pinetcore.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable pinetcore

echo "Installation complete. Run 'sudo systemctl start pinetcore'"

