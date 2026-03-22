use anyhow::Result;
use clap::Parser;
use tracing_subscriber::prelude::*;
use tracing::{info, Level};

mod config;
mod utils;
mod capture;
mod api;
mod firewall;
mod plugins;
mod storage;

use config::Config;
use storage::Database;
use api::routes::create_api_router;
use std::sync::Arc;
use axum::{Router, Server};
use tower_http::trace::TraceLayer;
use crate::plugins::{PluginManager, VpnPlugin, AdblockPlugin, IdsPlugin};

#[derive(Parser)]
struct Args {
    #[arg(long, default_value = "config.yaml")]
    config: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_target(false))
        .with(tracing_subscriber::EnvFilter::from_default_env()
            .add_directive(Level::INFO.into()))
        .init();

    let args = Args::parse();
    let config = Config::load(&args.config)?;

    info!("PiNetCore Core Engine v0.1.0 starting...");

    let db = Arc::new(Database::new(&config.database_url).await?);

    // Init plugins
    let mut plugin_mgr = PluginManager::new();
    plugin_mgr.register(Box::new(VpnPlugin::new(true)));
    plugin_mgr.register(Box::new(AdblockPlugin::new(true)));
    plugin_mgr.register(Box::new(IdsPlugin::new(false)));

    for plugin in &plugin_mgr.plugins {
        if let Err(e) = plugin.start() {
            tracing::warn!("Plugin {} failed to start: {}", plugin.name(), e);
        }
    }

    let app = Router::new()
        .nest("/api", create_api_router(db.clone()))
        .route("/metrics", axum::routing::get(api::handlers::get_metrics))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.api_port)).await?;
    let server = axum::serve(listener, app.into_make_service());

    let db_clone = db.clone();
    let capture_handle = tokio::spawn(async move {
        if let Err(e) = capture::start_capture(config.capture.clone(), db_clone).await {
            tracing::error!("Capture failed: {}", e);
        }
    });
    let server_handle = tokio::spawn(server);

    tokio::select! {
        _ = (&mut capture_handle) => {},
        _ = (&mut server_handle) => {},
    };

    for plugin in &plugin_mgr.plugins {
        let _ = plugin.stop();
    }

    info!("PiNetCore Core Engine stopped.");
    Ok(())
}

