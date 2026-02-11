use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;

/* =======================
   NETWORK LOGGING
   ======================= */
pub fn log_network_delta(interface: &str, sent: u64, received: u64) {
    let log = format!(
        "{} | iface={} | interval_sent={} | interval_received={}\n",
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

/* =======================
   DNS LOGGING
   ======================= */
pub fn log_dns_query(domain: &str) {
    let log = format!(
        "{} | domain={}\n",
        Local::now().to_rfc3339(),
        domain
    );

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("dns.log")
        .unwrap();

    file.write_all(log.as_bytes()).unwrap();
}

/* =======================
   APPLICATION LOGGING
   ======================= */
pub fn log_application(
    pid: i32,
    name: &str,
    cpu: f32,
    memory: u64,
    exe: Option<&str>,
) {
    let log = format!(
        "{} | pid={} | name={} | cpu={:.2}% | mem={} KB | exe={}\n",
        Local::now().to_rfc3339(),
        pid,
        name,
        cpu,
        memory,
        exe.unwrap_or("unknown"),
    );

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("applications.log")
        .unwrap();

    file.write_all(log.as_bytes()).unwrap();
}

