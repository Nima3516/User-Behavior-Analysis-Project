use crate::logger;
use systemd::journal;
use regex::Regex;

pub fn collect() {
    // Open systemd journal
    let mut journal = journal::Journal::open(journal::JournalFiles::All, false, false)
        .expect("Cannot open systemd journal");

    // Filter for systemd-resolved logs
    journal.match_add("SYSLOG_IDENTIFIER=systemd-resolved").unwrap();

    // Regex to extract domain from log line
    let domain_regex = Regex::new(r"query \[A\] (.+?) IN").unwrap();

    while let Some(entry) = journal.next_entry().unwrap() {
        if let Some(message) = entry.get("MESSAGE") {
            if let Some(cap) = domain_regex.captures(message) {
                let domain = &cap[1];
                println!("DNS QUERY: {}", domain);       // Print to terminal
                logger::log_dns_query(domain);          // Save to dns.log
            }
        }
    }
}
