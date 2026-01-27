use std::{thread, time::Duration};
use crate::network;
use crate::dns;

pub fn run() {
    loop {
        network::collect();
        dns::collect();
        thread::sleep(Duration::from_secs(5));
    }
}

