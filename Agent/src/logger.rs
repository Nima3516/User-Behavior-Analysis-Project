use chrono::Utc;
use std::{fs::OpenOptions, io::Write};

pub fn log_network(interface: &str, sent: u64, received: u64) {
    let log = format!(
        "{} | {} | sent={} | received={}\n",
        Utc::now().to_rfc3339(),
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
