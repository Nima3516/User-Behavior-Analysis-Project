use chrono::Local;
use std::{fs::OpenOptions, io::Write};

pub fn log_network_delta(interface: &str, sent: u64, received: u64) {
    let log = format!(
        "{} | {} | interval_sent={} | interval_received={}\n",
        Local::now().to_rfc3339(),
        interface,
        sent,
        received
    );

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("network.log")
        .unwrap();

    file.write_all(log.as_bytes()).unwrap();
}
