# PiNetCore -- Raspberry Pi Network Appliance

[![CI](https://github.com/tworjaga/PiNetCore/workflows/ci/badge.svg)](https://github.com/tworjaga/PiNetCore/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)](https://www.rust-lang.org)
[![Docker](https://img.shields.io/badge/Docker-ARM-blue.svg)](https://www.docker.com)
[![React](https://img.shields.io/badge/React-18-blue.svg)](https://react.dev)
<img src="https://visitor-badge.laobi.icu/badge?page_id=tworjaga.PiNetCore&"  />

## Overview

PiNetCore is a production-ready network appliance for Raspberry Pi 5. Full router control, real-time packet logging, nftables firewall, modular plugins (WireGuard VPN, Pi-hole DNS, Suricata IDS), React dashboard with live metrics.

## Architecture

```
Internet/WAN → Rust Core (pnet TAP/bridge + Axum API + SQLite) → Plugins → React Dashboard (Vite/Tailwind/Recharts)
```

## Key Features

- Packet capture/logging (Ethernet/IP/TCP/UDP → SQLite)
- nftables firewall (blacklist/whitelist)
- Plugin system (VPN/Pi-hole/Suricata ready)
- Axum REST + /metrics (Prometheus)
- React dashboard + real-time WebSocket
- Docker ARM + systemd deploy
- Pi5 LiteOS optimized

## Project Structure

```
PiNetCore/
├── core-engine/           # Rust backend (Tokio/Axum/pnet/sqlx)
│   ├── src/{api,capture,firewall,plugins,storage}
│   ├── Cargo.toml
│   └── tests/
├── dashboard/             # React frontend (Vite/Tailwind/React Query)
│   ├── src/{components,hooks,services,store}
│   ├── package.json
│   └── tests/
├── deployments/           # Docker + systemd + Pi scripts
│   ├── docker/
│   ├── scripts/install_pi.sh
│   └── docker-compose.yml
└── configs/               # nftables/wireguard
```

## Quick Start

### Docker (Recommended)
```bash
git clone https://github.com/tworjaga/PiNetCore
cd PiNetCore
docker compose up -d
```
API: http://localhost:8080/api/connections  
Dashboard: http://localhost:3000  
Metrics: http://localhost:8080/metrics

### Pi5 Native
```bash
./deployments/scripts/install_pi.sh
sudo systemctl start pinetcore
```

## Compatibility

| Router | Bridge Mode | VLAN | ISP Modem |
|--------|-------------|------|-----------|
| ASUS/TP-Link/Netgear | Yes | Yes | Yes |
| OpenWrt/DD-WRT | Yes | Yes | Yes |
| ISP Locked | Manual | No | No |

## Technology Stack

**Backend (Rust)**  
Runtime: Tokio  
Web: Axum  
Packet: pnet  
DB: sqlx (SQLite)  
Firewall: nftables bindings  

**Frontend (TypeScript)**  
Framework: Vite + React 18  
Styling: Tailwind CSS  
Charts: Recharts  
Query: TanStack Query  

**Infrastructure**  
Container: Docker ARM64  
Compose: Multi-service stack  
Deploy: systemd service  

## Testing

```bash
# Backend
cd core-engine
cargo test           # Unit/integration
cargo clippy         # Lints

# Frontend
cd dashboard
npm test             # Vitest (100% coverage)
npm run build        # Production build
```

## Configuration

`.env` or `core-engine/config.yaml`:
```
database_url = "sqlite://pinetcore.db"
api_port = 8080
capture:
  interface = "eth0"
firewall:
  enabled = true
```

## API

**REST**:
```
GET /api/connections    # Recent connections
GET /health             # Status
GET /metrics            # Prometheus
```

**Example**:
```bash
curl http://localhost:8080/api/connections
```

## Deployment

**Docker Compose** (Grafana/Prometheus/Loki ready):
```bash
docker compose -f docker-compose.monitoring.yml up -d
```

**Pi Production**:
1. `./deployments/scripts/install_pi.sh`
2. `sudo systemctl enable pinetcore`
3. `sudo systemctl status pinetcore`

## Plugins

```rust
pub struct MyPlugin;
impl Plugin for MyPlugin {
    fn name(&self) -> &str { "MyPlugin" }
    fn start(&self) -> anyhow::Result<()> { Ok(()) }
}
```

Register in `main.rs` → auto start/stop.

## Contributing

See CONTRIBUTING.md for guidelines on:
1. Fork → feature branch
2. `cargo fmt + cargo clippy`
3. PR with tests

## License

MIT License - see LICENSE for details.

---

Repository: https://github.com/tworjaga/PiNetCore  
Telegram: @al7exy

