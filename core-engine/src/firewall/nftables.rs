use std::process::Command;

#[derive(Debug)]
pub struct Nftables {
    path: String,
}

impl Nftables {
    pub fn new(path: String) -> Self {
        Self { path }
    }

    pub fn load_config(&self, config_path: &str) -> anyhow::Result<()> {
        let status = Command::new(&self.path)
            .arg("-f")
            .arg(config_path)
            .status()?;
        if status.success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("nftables load failed"))
        }
    }

    pub fn add_rule(&self, table: &str, chain: &str, rule: &str) -> anyhow::Result<()> {
        let status = Command::new(&self.path)
            .arg("add")
            .arg("rule")
            .arg(table)
            .arg(chain)
            .arg(rule)
            .status()?;
        status.success().then_some(()).ok_or(anyhow::anyhow!("add rule failed"))
    }
}

