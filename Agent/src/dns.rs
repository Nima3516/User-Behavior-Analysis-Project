use chrono::Utc;
use pnet::datalink::{self, Channel::Ethernet};
use pnet::packet::{ethernet::EthernetPacket, ip::IpNextHeaderProtocols, ipv4::Ipv4Packet, udp::UdpPacket, Packet};
use regex::Regex;
use crate::logger;

pub fn collect() {
    // Pick the interface (replace "enp2s0" with your real interface if different)
    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .find(|iface| iface.is_up() && !iface.is_loopback() && iface.name == "enp2s0")
        .expect("No valid network interface found");

    // Create channel
    let channel = datalink::channel(&interface, Default::default())
        .expect("Failed to create datalink channel");

    // Extract receiver
    let mut rx = match channel {
        Ethernet(_, rx) => rx,
        _ => panic!("Unhandled channel type"),
    };

    // Regex to extract domain names from DNS payloads
    let domain_regex = Regex::new(r"([a-zA-Z0-9\-]+\.)+[a-zA-Z]{2,}").unwrap();

    loop {
        if let Ok(packet) = rx.next() {
            if let Some(eth) = EthernetPacket::new(packet) {
                if let Some(ipv4) = Ipv4Packet::new(eth.payload()) {
                    if ipv4.get_next_level_protocol() == IpNextHeaderProtocols::Udp {
                        if let Some(udp) = UdpPacket::new(ipv4.payload()) {
                            // DNS queries use port 53
                            if udp.get_destination() == 53 {
                                let payload = udp.payload();
                                if let Ok(text) = std::str::from_utf8(payload) {
                                    for cap in domain_regex.captures_iter(text) {
                                        let domain = &cap[0];
                                        println!("DNS QUERY: {}", domain);
                                        logger::log_dns_query(domain);
                                    }
                                }
                            } // end if udp port 53
                        } // end if udp packet
                    } // end if udp protocol
                } // end if ipv4 packet
            } // end if ethernet packet
        } // end if rx.next()
    } // end loop
} // end fn

