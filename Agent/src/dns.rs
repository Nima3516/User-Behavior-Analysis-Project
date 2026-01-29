use chrono::Utc;
use systemd::journal;
use crate::logger;

pub fn collect() {
    let mut journal = journal::Journal::open(journal::JournalFiles::All, false, false)
        .expect("Cannot open systemd journal");

    // Filter for systemd-resolved messages
    journal.match_add("SYSLOG_IDENTIFIER=systemd-resolved").unwrap();

    while let Some(entry) = journal.next_entry().unwrap() {
        if let Some(message) = entry.get("MESSAGE") {
            if message.contains("query") {
                println!("DNS QUERY: {}", message);
                // Optionally log to file
                // logger::log_dns_query(message);
            }
        }
    }
}
