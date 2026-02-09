use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;

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


