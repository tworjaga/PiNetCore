pub mod vpn;
pub mod adblock;
pub mod ids;

pub trait Plugin {
    fn name(&self) -> &str;
    fn start(&self) -> anyhow::Result<()>;
    fn stop(&self) -> anyhow::Result<()>;
}

pub struct PluginManager {
    plugins: Vec<Box<dyn Plugin + Send + Sync>>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self { plugins: vec![] }
    }

    pub fn register(&mut self, plugin: Box<dyn Plugin + Send + Sync>) {
        self.plugins.push(plugin);
    }
}

