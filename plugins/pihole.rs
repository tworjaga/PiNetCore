// Move to core-engine/src/plugins/pihole.rs
use crate::plugins::Plugin;
use std::process::Command;

pub struct PiHolePlugin;

impl Plugin for PiHolePlugin {
    fn name(&self) -> &str {
        "Pi-hole"
    }

    fn start(&self) -> anyhow::Result<()> {
        Command::new("docker")
            .args(&["run", "-d", "--name", "pihole", 
                   "-p", "53:53/tcp", "-p", "53:53/udp", "-p", "80:80",
                   "-e", "WEBPASSWORD=admin", "pihole/pihole"])
            .status()?;
        Ok(())
    }

    fn stop(&self) -> anyhow::Result<()> {
        Command::new("docker").args(&["stop", "pihole"]).status()?;
        Ok(())
    }
}

