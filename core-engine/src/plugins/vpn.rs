use crate::plugins::Plugin;

pub struct VpnPlugin {
    enabled: bool,
}

impl VpnPlugin {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}

impl Plugin for VpnPlugin {
    fn name(&self) -> &str {
        "VPN (WireGuard)"
    }

    fn start(&self) -> anyhow::Result<()> {
        if !self.enabled {
            return Ok(());
        }
        tracing::info!("Starting VPN plugin");
        // wg-quick up wg0 placeholder
        Ok(())
    }

    fn stop(&self) -> anyhow::Result<()> {
        tracing::info!("Stopping VPN plugin");
        Ok(())
    }
}

