use crate::plugins::Plugin;

pub struct AdblockPlugin {
    enabled: bool,
}

impl AdblockPlugin {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}

impl Plugin for AdblockPlugin {
    fn name(&self) -> &str {
        "Pi-hole Adblock"
    }

    fn start(&self) -> anyhow::Result<()> {
        if !self.enabled {
            return Ok(());
        }
        tracing::info!("Starting Pi-hole integration");
        // docker run -d pihole/pihole stub
        Ok(())
    }

    fn stop(&self) -> anyhow::Result<()> {
        tracing::info!("Stopping Pi-hole");
        Ok(())
    }
}

