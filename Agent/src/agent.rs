use std::{thread, time::Duration};
use crate::network;

pub fn run() {
    loop {
        network::collect();
        thread::sleep(Duration::from_secs(5));
    }
}
