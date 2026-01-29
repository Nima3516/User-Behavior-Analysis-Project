use pnet::datalink::{self, Channel::Ethernet};
use pnet::packet::{ethernet::EthernetPacket, ip::IpNextHeaderProtocols, ipv4::Ipv4Packet, udp::UdpPacket, Packet};
use dns_parser::Packet as DnsPacket;
use crate::logger;

pub fn collect() {
    // Find your network interface (replace "enp2s0" with your actual interface if needed)
    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .find(|iface| iface.is_up() && !iface.is_loopback() && iface.name == "enp2s0")
        .expect("No valid network interface found");

    // Create a channel to listen to packets
    let channel = datalink::channel(&interface, Default::default())
        .expect("Failed to create datalink channel");

    let mut rx = match channel {
        Ethernet(_, rx) => rx,
        _ => panic!("Unhandled channel type"),
    };

    loop {
        if let Ok(packet) = rx.next() {
            if let Some(eth) = EthernetPacket::new(packet) {
                if let Some(ipv4) = Ipv4Packet::new(eth.payload()) {
                    if ipv4.get_next_level_protocol() == IpNextHeaderProtocols::Udp {
                        if let Some(udp) = UdpPacket::new(ipv4.payload()) {
                            // DNS queries are usually on port 53
                            if udp.get_destination() == 53 || udp.get_source() == 53 {
                                if let Ok(dns_packet) = DnsPacket::parse(udp.payload()) {
                                    // Log all queried domain names
                                    for question in dns_packet.questions {
                                        let domain = question.qname.to_string();
                                        println!("DNS QUERY: {}", domain);
                                        logger::log_dns_query(&domain);
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



