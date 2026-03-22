use anyhow::Result;
use pnet::datalink::NetworkInterface;
use std::process::Command;

pub fn setup_bridge(interface1: &str, interface2: &str) -> Result<()> {
    // Linux bridge setup
    Command::new("ip").args(&["link", "add", "name", "br0", "type", "bridge"]).status()?;
    Command::new("ip").args(&["link", "set", interface1, "master", "br0"]).status()?;
    Command::new("ip").args(&["link", "set", interface2, "master", "br0"]).status()?;
    Command::new("ip").args(&["link", "set", "br0", "up"]).status()?;
    Ok(())
}

pub fn teardown_bridge() -> Result<()> {
    Command::new("ip").arg("link").arg("delete").arg("br0").status()?;
    Ok(())
}

