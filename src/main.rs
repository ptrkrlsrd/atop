use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use sysinfo::{ProcessExt, System, SystemExt};

const THRESHOLD: f32 = 0.1;

#[derive(Debug)]
struct ProcessInfo {
    cpu_usage_sum: f32,
    cpu_usage: f32,
    memory_usage_sum: u64,
    memory_usage: u64,
    count: u64,
}

fn update_process_info(sys: &System, process_info_map: &mut HashMap<String, ProcessInfo>) {
    for (_pid, proc) in sys.processes() {
        let entry = process_info_map
            .entry(proc.name().to_string())
            .or_insert_with(|| ProcessInfo {
                cpu_usage_sum: 0.0,
                memory_usage_sum: 0,
                count: 0,
                cpu_usage: 0.0,
                memory_usage: 0,
            });
        entry.cpu_usage_sum += proc.cpu_usage();
        entry.cpu_usage = proc.cpu_usage();
        entry.memory_usage_sum += proc.memory();
        entry.memory_usage = proc.memory();
        entry.count += 1;
    }
}

fn sort_by_cpu_usage(process_vec: &mut Vec<(&String, &ProcessInfo)>) {
    process_vec.sort_by(|a, b| {
        let avg_cpu_usage_a = a.1.cpu_usage_sum / a.1.count as f32;
        let avg_cpu_usage_b = b.1.cpu_usage_sum / b.1.count as f32;
        avg_cpu_usage_b
            .partial_cmp(&avg_cpu_usage_a)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
}

fn main() {
    let delay = Duration::from_millis(1000);
    let mut sys = System::new_all();
    let mut process_info_map: HashMap<String, ProcessInfo> = HashMap::new();

    loop {
        print!("\x1B[2J\x1B[1;1H");

        update_process_info(&sys, &mut process_info_map);

        let mut process_vec: Vec<(&String, &ProcessInfo)> = process_info_map.iter().collect();
        sort_by_cpu_usage(&mut process_vec);

        sys.refresh_all();

        println!(
            "{:<45} | {:<11} | {:<10} | {:<10} | {:<20}",
            "PID", "Avg CPU", "CPU", "Avg RAM", "RAM"
        );

        for (pid, info) in &process_vec {
            let cpu = info.cpu_usage;
            let memory_usage = info.memory_usage / 1024;
            let avg_cpu_usage = info.cpu_usage_sum / info.count as f32;
            let avg_memory_usage = memory_usage / info.count;
            if avg_cpu_usage < THRESHOLD {
                continue;
            }

            println!(
                "{:<45} | {:<10.2}% | {:<10.2} | {:<11}| {:<20}",
                pid,
                avg_cpu_usage,
                cpu,
                format!("{}M", avg_memory_usage),
                format!("{}M", memory_usage)
            );
        }

        thread::sleep(delay);
    }
}
