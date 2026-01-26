use std::collections::HashMap;
use std::thread;
use std::time::Duration;

use sysinfo::{Networks, System};

use crate::logger::log_network_delta;

pub fn collect() {
    let mut sys = System::new_all();

    // Store previous counters: interface -> (rx, tx)
    let mut previous: HashMap<String, (u64, u64)> = HashMap::new();

    loop {
        // Refresh network statistics
        sys.refresh_networks();

        let networks: &Networks = sys.networks();

        for (iface, data) in networks.iter() {
            let current_rx = data.received();
            let current_tx = data.transmitted();

            if let Some((prev_rx, prev_tx)) = previous.get(iface) {
                let interval_rx = current_rx.saturating_sub(*prev_rx);
                let interval_tx = current_tx.saturating_sub(*prev_tx);

                // 🔹 Use YOUR logger function
                log_network_delta(iface, interval_tx, interval_rx);
            }

            // Update previous values
            previous.insert(iface.to_string(), (current_rx, current_tx));
        }

        thread::sleep(Duration::from_secs(5));
    }
}



