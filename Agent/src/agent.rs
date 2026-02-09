use std::thread;
use std::time::Duration;

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
        thread::sleep(Duration::from_secs(60));
    }
}

