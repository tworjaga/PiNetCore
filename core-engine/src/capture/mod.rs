use crate::config::CaptureConfig;
use pnet::datalink::{self, NetworkInterface};
use tokio::sync::mpsc;

use crate::capture::packet::process_packet;
use pnet::datalink::{self, Channel, Config};
use std::sync::Arc;
use crate::storage::Database;
use tokio::sync::mpsc;

pub async fn start_capture(config: CaptureConfig, db: Arc<Database>) -> anyhow::Result<()> {
    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .find(|iface| iface.name == config.interface)
        .ok_or(anyhow::anyhow!("Interface not found"))?;

    let (_tx, rx) = match datalink::channel(&interface, Default::default()) {
        Ok(datalink::Channel::Ethernet(tx, mut rx)) => {
            let (packet_tx, packet_rx) = mpsc::channel(1024);
            tokio::spawn(async move {
                while let Ok(packet) = rx.next() {
                    let _ = packet_tx.send(packet).await;
                }
            });
            (packet_tx, packet_rx)
        }
        Ok(_) => return Err(anyhow::anyhow!("Unsupported channel")),
        Err(e) => return Err(anyhow::anyhow!("Channel error: {}", e)),
    };

    while let Some(packet) = rx.recv().await {
        process_packet(db.clone(), &packet).await;
    }

    tracing::info!("Packet capture started on {}", config.interface);
    Ok(())
}

