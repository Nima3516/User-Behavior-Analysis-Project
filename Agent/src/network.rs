use std::collections::HashMap;
use std::thread;
use std::time::Duration;

use sysinfo::Networks;

use crate::logger::log_network_delta;

pub fn collect() {
    // Create Networks object (NOT System)
    let mut networks = Networks::new_with_refreshed_list();

    // Store previous counters: interface -> (rx, tx)
    let mut previous: HashMap<String, (u64, u64)> = HashMap::new();

    loop {
        // Refresh network data
        networks.refresh();

        for (iface, data) in networks.iter() {
            let current_rx = data.received();
            let current_tx = data.transmitted();

            if let Some((prev_rx, prev_tx)) = previous.get(iface) {
                let interval_rx = current_rx.saturating_sub(*prev_rx);
                let interval_tx = current_tx.saturating_sub(*prev_tx);

                log_network_delta(iface, interval_tx, interval_rx);
            }

            previous.insert(iface.to_string(), (current_rx, current_tx));
        }

        thread::sleep(Duration::from_secs(5));
    }
}




