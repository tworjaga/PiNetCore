#!/bin/bash
# OTA update stub

curl -s https://api.github.com/repos/tworjaga/PiNetCore/releases/latest | jq -r .tag_name > /tmp/latest_version
CURRENT=$(grep version Cargo.toml | head -1 | cut -d'"' -f2)

if [ "$CURRENT" != "$LATEST" ]; then
  git pull origin main
  cargo build --release --target aarch64-unknown-linux-gnu
  sudo systemctl restart pinetcore
  echo "Updated to $LATEST"
fi

