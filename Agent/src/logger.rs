use chrono::Local;
use std::{fs::OpenOptions, io::Write};

/*pub fn log_network_delta(interface: &str, sent: u64, received: u64) {
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
} */
pub fn log_dns_query(domain: &str) {
    let log = format!(
        "{} | {}\n",
        chrono::Utc::now().to_rfc3339(),
        domain
    );

    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("dns.log")
        .unwrap();

    file.write_all(log.as_bytes()).unwrap();
}

pub fn log_application(
    pid: i32,
    name: &str,
    cpu: f32,
    memory: u64,
    exe: Option<&str>,
) {
    let log = format!(
        "{} | pid={} | name={} | cpu={} | mem={} KB | exe={}\n",
        chrono::Local::now().to_rfc3339(),
        pid,
        name,
        cpu,
        memory,
        exe.unwrap_or("unknown"),
    );

    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("applications.log")
        .unwrap();

    file.write_all(log.as_bytes()).unwrap();
}



