use pnet::datalink::{self, Channel::Ethernet};
use pnet::packet::{
    ethernet::EthernetPacket,
    ip::IpNextHeaderProtocols,
    ipv4::Ipv4Packet,
    udp::UdpPacket,
    Packet,
};

pub fn collect() {
    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .find(|iface| iface.is_up() && !iface.is_loopback())
        .expect("No valid interface found");

    let mut rx = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(_, rx)) => rx,
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("Failed to create channel: {}", e),
    };

    if let Ok(packet) = rx.next() {
        if let Some(eth) = EthernetPacket::new(packet) {
            if let Some(ipv4) = Ipv4Packet::new(eth.payload()) {
                if ipv4.get_next_level_protocol() == IpNextHeaderProtocols::Udp {
                    if let Some(udp) = UdpPacket::new(ipv4.payload()) {
                        if udp.get_destination() == 53 {
                            println!("DNS packet detected (query)");
                        }
                    }
                }
            }
        }
    }
}
