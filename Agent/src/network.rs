use std::collections::HashMap;
use std::thread;
use std::time::Duration;

use chrono::Utc;
use sysinfo::{Networks, System};

use crate::logger::write_log; // 👈 change if your logger uses a different name

pub fn collect() {
    let mut sys = System::new_all();

    // This holds previous rx/tx values
    let mut previous: HashMap<String, (u64, u64)> = HashMap::new();

    loop {
        sys.refresh_networks();

        let networks: &Networks = sys.networks();

        for (iface, data) in networks.iter() {
            let current_rx = data.received();
            let current_tx = data.transmitted();

            if let Some((prev_rx, prev_tx)) = previous.get(iface) {
                let delta_rx = current_rx.saturating_sub(*prev_rx);
                let delta_tx = current_tx.saturating_sub(*prev_tx);

                let line = format!(
                    "{} | {} | interval_sent={} | interval_received={}",
                    Utc::now(),
                    iface,
                    delta_tx,
                    delta_rx
                );

                write_log(&line);
            }

            previous.insert(iface.to_string(), (current_rx, current_tx));
        }

        thread::sleep(Duration::from_secs(5));
    }
}



