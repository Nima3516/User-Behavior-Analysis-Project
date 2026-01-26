use sysinfo::{Networks, System};
use crate::logger;
use std::collections::HashMap;

// Store previous values for each interface
static mut PREVIOUS: Option<HashMap<String, (u64, u64)>> = None;

pub fn collect() {
    let mut sys = System::new_all();
    let mut networks = Networks::new_with_refreshed_list();
    networks.refresh();

    unsafe {
        if PREVIOUS.is_none() {
            PREVIOUS = Some(HashMap::new());
        }

        let prev_map = PREVIOUS.as_mut().unwrap();

        for (name, data) in networks.iter() {
            let current_sent = data.transmitted();
            let current_received = data.received();

            let (prev_sent, prev_received) =
                prev_map.get(name).cloned().unwrap_or((current_sent, current_received));

            let delta_sent = current_sent - prev_sent;
            let delta_received = current_received - prev_received;

            logger::log_network_delta(
                name,
                delta_sent,
                delta_received,
            );

            prev_map.insert(name.to_string(), (current_sent, current_received));
        }
    }
}
