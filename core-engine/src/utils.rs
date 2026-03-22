use chrono::{DateTime, Utc};

pub fn now() -> DateTime<Utc> {
    Utc::now()
}

pub fn log_connection(ip: &str, port: u16, device: &str) {
    let timestamp = now().to_rfc3339();
    tracing::info!(ip, port, device, timestamp, "New connection logged");
}

pub fn parse_interface_name(name: &str) -> anyhow::Result<String> {
    if name.is_empty() {
        anyhow::bail!("Interface name cannot be empty");
    }
    Ok(name.to_string())
}

