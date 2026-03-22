use pnet::packet::Packet;
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;
use crate::storage::Database;
use crate::utils::log_connection;

pub async fn process_packet(db: std::sync::Arc<Database>, packet_data: &[u8]) {
    if let Some(eth) = EthernetPacket::new(packet_data) {
        if let Some(ipv4) = Ipv4Packet::new(eth.payload()) {
            let src_ip = ipv4.get_source().to_string();
            let dst_ip = ipv4.get_destination().to_string();
            let protocol = ipv4.get_next_level_protocol();

            let (src_port, dst_port) = match protocol {
                IpNextHeaderProtocols::Tcp => {
                    if let Some(tcp) = TcpPacket::new(ipv4.payload()) {
                        (tcp.get_source(), tcp.get_destination())
                    } else {
                        (0, 0)
                    }
                }
                IpNextHeaderProtocols::Udp => {
                    if let Some(udp) = UdpPacket::new(ipv4.payload()) {
                        (udp.get_source(), udp.get_destination())
                    } else {
                        (0, 0)
                    }
                }
                _ => (0, 0),
            };

            log_connection(&src_ip, src_port, "unknown").await;
            if let Err(e) = db.log_connection(&src_ip, src_port as u16, None).await {
                tracing::error!("Failed to log connection: {}", e);
            }
        }
    }
}

