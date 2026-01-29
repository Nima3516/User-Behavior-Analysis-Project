use chrono::Utc;
use pnet::datalink::{self, Channel::Ethernet};
use pnet::packet::{ethernet::EthernetPacket, ip::IpNextHeaderProtocols, ipv4::Ipv4Packet, udp::UdpPacket, Packet};
use regex::Regex;
use crate::logger;

pub fn collect() {
    // Choose the first active, non-loopback interface
    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .find(|iface| iface.is_up() && !iface.is_loopback() && iface.name == "enp2s0")
        .expect("No valid network interface found");

    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(_, rx)) => rx,
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("Failed to create channel: {}", e),
    };

    // Regex to extract domain names from DNS query packets
    let domain_regex = Regex::new(r"([a-zA-Z0-9\-]+\.)+[a-zA-Z]{2,}").unwrap();

    loop {
        if let Ok(packet) = rx.next() {
            if let Some(eth) = EthernetPacket::new(packet) {
                if let Some(ipv4) = Ipv4Packet::new(eth.payload()) {
                    if ipv4.get_next_level_protocol() == IpNextHeaderProtocols::Udp {
                        if let Some(udp) = UdpPacket::new(ipv4.payload()) {
                            // DNS queries use destination port 53
                            if udp.get_destination() == 53 {
                                let payload = udp.payload();
                                // Convert payload to printable string
                                if let Ok(text) = std::str::from_utf8(payload) {
                                    for cap in domain_regex.captures_iter(text) {
                                        let domain = &cap[0];
                                        println!("DNS QUERY: {}", domain);
                                        logger::log_dns_query(domain);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

