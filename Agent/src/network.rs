use std::collections::HashMap;
use std::thread;
use std::time::Duration;

use chrono::Utc;
use sysinfo::{NetworkExt, NetworksExt, System, SystemExt};

use crate::logger::log_line;

pub fn start_network_monitor() {
    // Create system object ONCE
    let mut sys = System::new_all();

    // Store previous counters here (interface -> (rx, tx))
    let mut previous: HashMap<String, (u64, u64)> = HashMap::new();

    loop {
        // Refresh network statistics
        sys.refresh_networks();

        for (iface, data) in sys.networks() {
            let current_rx = data.received();
            let current_tx = data.transmitted();

            // If we have previous data, calculate interval usage
            if let Some((prev_rx, prev_tx)) = previous.get(iface) {
                let interval_rx = current_rx.saturating_sub(*prev_rx);
                let interval_tx = current_tx.saturating_sub(*prev_tx);

                let timestamp = Utc::now();

                let line = format!(
                    "{} | {} | interval_sent={} | interval_received={}",
                    timestamp,
                    iface,
                    interval_tx,
                    interval_rx
                );

                log_line(&line);
            }

            // Update previous values
            previous.insert(iface.to_string(), (current_rx, current_tx));
        }

        // Wait before next sample
        thread::sleep(Duration::from_secs(5));
    }
}


