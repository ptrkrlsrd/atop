use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use structopt::StructOpt;
use sysinfo::{ProcessExt, System, SystemExt};

#[derive(StructOpt, Debug)]
struct Cli {
    #[structopt(short, long, about = "Minimum threshold of CPU usage in %")]
    threshold: Option<f32>,
}

#[derive(Debug, Clone)]
struct ProcessInfo {
    cpu_usage_sum: f32,
    cpu_usage: f32,
    memory_usage_sum: u64,
    memory_usage: u64,
    count: u64,
}

#[derive(Debug, Clone)]
struct Process {
    pid: String,
    info: ProcessInfo,
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

fn sort_by_cpu_usage(process_vec: &mut Vec<Process>) {
    process_vec.sort_by(|a, b| {
        let avg_cpu_usage_a = a.info.cpu_usage_sum / a.info.count as f32;
        let avg_cpu_usage_b = b.info.cpu_usage_sum / b.info.count as f32;
        avg_cpu_usage_b
            .partial_cmp(&avg_cpu_usage_a)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
}

fn print_process_info(process_vec: &[Process], threshold: f32) {
    println!(
        "{:<45} | {:<11} | {:<10} | {:<10} | {:<20}",
        "PID", "Avg CPU", "CPU", "Avg RAM", "RAM"
    );
    for process in process_vec {
        let cpu = process.info.cpu_usage;
        let memory_usage = process.info.memory_usage / 1024;
        let avg_cpu_usage = process.info.cpu_usage_sum / process.info.count as f32;
        let avg_memory_usage = memory_usage / process.info.count;
        if avg_cpu_usage < threshold {
            continue;
        }
        println!(
            "{:<45} | {:<10.2}% | {:<10.2} | {:<11}| {:<20}",
            process.pid,
            avg_cpu_usage,
            cpu,
            format!("{}M", avg_memory_usage),
            format!("{}M", memory_usage)
        );
    }
}

fn main() {
    let delay = Duration::from_millis(1000);
    let mut sys = System::new_all();
    let mut process_info_map: HashMap<String, ProcessInfo> = HashMap::new();

    let args = Cli::from_args();

    let threshold = args.threshold.unwrap_or(0.1);

    if threshold < 0.0 || threshold > 100.0 {
        panic!("Threshold should be between 0 and 100");
    }

    loop {
        print!("\x1B[2J\x1B[1;1H");
        sys.refresh_all();

        update_process_info(&sys, &mut process_info_map);

        let mut process_vec: Vec<Process> = process_info_map
            .iter()
            .map(|i| Process {
                pid: i.0.clone(),
                info: i.1.clone(),
            })
            .collect();
        sort_by_cpu_usage(&mut process_vec);

        print_process_info(&process_vec, threshold);

        thread::sleep(delay);
    }
}
