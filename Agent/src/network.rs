use sysinfo::{NetworksExt, System, SystemExt};
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use crate::logger;

pub fn collect() {
    println!("Network bandwidth monitoring started");

    let mut sys = System::new_all();
    let mut previous: HashMap<String, (u64, u64)> = HashMap::new();

    loop {
        sys.refresh_networks();

        for (iface, data) in sys.networks() {
            let sent = data.transmitted();
            let received = data.received();

            let entry = previous.entry(iface.to_string()).or_insert((sent, received));

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


