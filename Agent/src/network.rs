use sysinfo::{Networks, System};
use crate::logger;

pub fn collect() {
    let mut sys = System::new_all();
    let mut networks = Networks::new_with_refreshed_list();

    networks.refresh();

    for (name, data) in networks.iter() {
        logger::log_network(
            name,
            data.transmitted(),
            data.received(),
        );
    }
}
