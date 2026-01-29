use pnet::datalink::{self, Channel::Ethernet};
use pnet::packet::ethernet::EthernetPacket;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::udp::UdpPacket;
use pnet::packet::Packet;
use pnet::packet::ip::IpNextHeaderProtocols;
use crate::logger::log_dns_query;

pub fn collect() {
    // Get the default network interface
    let interfaces = datalink::interfaces();
    let interface = interfaces.into_iter()
        .find(|iface| iface.is_up() && !iface.is_loopback() && !iface.ips.is_empty())
        .expect("No active interface found");

    // Create a data link channel
    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("Failed to create channel: {}", e),
    };

    println!("Listening for DNS packets on {}", interface.name);

    loop {
        if let Ok(packet) = rx.next() {
            if let Some(eth) = EthernetPacket::new(packet) {
                if let Some(ipv4) = Ipv4Packet::new(eth.payload()) {
                    // Only UDP (DNS mostly uses UDP)
                    if ipv4.get_next_level_protocol() == IpNextHeaderProtocols::Udp {
                        if let Some(udp) = UdpPacket::new(ipv4.payload()) {
                            // Check if source or dest port is 53 (DNS)
                            if udp.get_source() == 53 || udp.get_destination() == 53 {
                                // DNS packet detected
                                let dns_data = udp.payload();
                                if let Ok(domain) = parse_dns_name(dns_data) {
                                    log_dns_query(&domain);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// Simple DNS parser for queries (extract first domain)
fn parse_dns_name(packet: &[u8]) -> Result<String, &'static str> {
    if packet.len() < 12 {
        return Err("Packet too short for DNS header");
    }

    let qdcount = u16::from_be_bytes([packet[4], packet[5]]);
    if qdcount == 0 {
        return Err("No questions in DNS packet");
    }

    let mut idx = 12; // DNS header is 12 bytes
    let mut labels = Vec::new();

    loop {
        if idx >= packet.len() {
            return Err("Unexpected end of packet");
        }
        let len = packet[idx] as usize;
        if len == 0 {
            break;
        }
        idx += 1;
        if idx + len > packet.len() {
            return Err("Invalid label length");
        }
        let label = std::str::from_utf8(&packet[idx..idx + len]).map_err(|_| "Invalid UTF-8")?;
        labels.push(label.to_string());
        idx += len;
    }

    Ok(labels.join("."))
}


