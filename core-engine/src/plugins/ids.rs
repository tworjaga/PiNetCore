use crate::plugins::Plugin;

pub struct IdsPlugin {
    enabled: bool,
}

impl IdsPlugin {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}

impl Plugin for IdsPlugin {
    fn name(&self) -> &str {
        "Suricata IDS"
    }

    fn start(&self) -> anyhow::Result<()> {
        if !self.enabled {
            return Ok(());
        }
        tracing::info!("Starting Suricata IDS");
        Ok(())
    }

    fn stop(&self) -> anyhow::Result<()> {
        tracing::info!("Stopping Suricata");
        Ok(())
    }
}

