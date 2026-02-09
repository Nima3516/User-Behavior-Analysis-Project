use std::{thread, time::Duration};
use crate::network;
use crate::dns;
use std::thread;

pub fn run() {
    println!("Agent is running...");

    thread::spawn(|| {
        crate::network::collect();
    });

    thread::spawn(|| {
        crate::dns::collect();
    });

    thread::spawn(|| {
        crate::app_monitor::collect();
    });

    loop {
        std::thread::sleep(std::time::Duration::from_secs(60));
    }
}

