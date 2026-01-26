use sysinfo::Networks;
use crate::logger;
use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;

// Global, safe, thread-safe storage
static PREVIOUS: Lazy<Mutex<HashMap<String, (u64, u64)>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub fn collect() {
    let mut networks = Networks::new_with_refreshed_list();
    networks.refresh();

    let mut prev_map = PREVIOUS.lock().unwrap();

    for (name, data) in networks.iter() {
        let current_sent = data.transmitted();
        let current_received = data.received();

        let (prev_sent, prev_received) =
            prev_map.get(name).cloned().unwrap_or((current_sent, current_received));

        let delta_sent = current_sent - prev_sent;
        let delta_received = current_received - prev_received;

        logger::log_network_delta(name, delta_sent, delta_received);

        prev_map.insert(name.to_string(), (current_sent, current_received));
    }
}


