use sysinfo::Networks;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use crate::logger;

pub fn collect() {
    println!("Network bandwidth monitoring started...");

    let mut networks = Networks::new_with_refreshed_list();
    let mut previous: HashMap<String, (u64, u64)> = HashMap::new();

    loop {
        networks.refresh();

        for (iface, data) in &networks {
            let sent = data.total_transmitted();
            let received = data.total_received();

            let entry = previous
                .entry(iface.to_string())
                .or_insert((sent, received));

            let delta_sent = sent - entry.0;
            let delta_received = received - entry.1;

            if delta_sent > 0 || delta_received > 0 {
                logger::log_network_delta(
                    iface,
                    delta_sent,
                    delta_received,
                );
            }

            *entry = (sent, received);
        }

        thread::sleep(Duration::from_secs(5));
    }
}
