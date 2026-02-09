use sysinfo::{System, SystemExt, ProcessExt};
use std::{collections::HashSet, thread, time::Duration};
use crate::logger::log_application;

pub fn collect() {
    let mut system = System::new_all();
    let mut seen: HashSet<i32> = HashSet::new();

    loop {
        system.refresh_processes();

        for (pid, process) in system.processes() {
            let pid_i32 = pid.as_u32() as i32;

            // Log only NEW processes (ghost detection)
            if !seen.contains(&pid_i32) {
                seen.insert(pid_i32);

                log_application(
                    pid_i32,
                    process.name(),
                    process.cpu_usage(),
                    process.memory(),
                    process.exe().and_then(|p| p.to_str()),
                );
            }
        }

        thread::sleep(Duration::from_secs(5));
    }
}
