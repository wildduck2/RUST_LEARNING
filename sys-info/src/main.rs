mod cpu;
mod ram;
mod utils;
use core::time;
use std::{path::Path, thread};

use sysinfo::System;

fn main() {
    let mut sys = System::new_all();

    loop {
        sys.refresh_all();
        // Clear the screen
        print!("\x1B[2J\x1B[1;1H");

        // CPU print information.
        cpu::get_cpu_info(&sys);
        // RAM print information.
        ram::get_swap_info(&sys);

        get_process_info(&sys);

        // Sleep the thread for a bit.
        thread::sleep(time::Duration::from_millis(1000));
    }
}

pub fn get_process_info(sys: &System) {
    // Table header
    println!(
        "  +{:-<6}+{:-<20}+{:-<9}+{:-<12}+{:-<12}+{:-<8}+{:-<40}+",
        "", "", "", "", "", "", ""
    );
    println!(
        "  | {:<6} | {:<18} | {:>7} | {:>10} | {:>10} | {:<6} | {:<38} |",
        "PID", "Name", "CPU (%)", "Memory", "Virtual", "Status", "Path"
    );
    println!(
        "  +{:-<6}+{:-<20}+{:-<9}+{:-<12}+{:-<12}+{:-<8}+{:-<40}+",
        "", "", "", "", "", "", ""
    );

    // Sort processes by CPU usage descending
    let mut processes: Vec<_> = sys.processes().iter().collect();
    processes.sort_by(|a, b| b.1.cpu_usage().partial_cmp(&a.1.cpu_usage()).unwrap());

    for (pid, process) in processes.iter().take(15) {
        let cpu_usage = process.cpu_usage() / 10.0;
        let mem = format_bytes(process.memory() * 1024); // sysinfo returns KB, convert to bytes
        let status = format!("{:?}", process.status());
        let name = truncate(process.name().to_str().unwrap(), 18);
        let path = process.exe().unwrap_or(Path::new(""));

        println!(
            "  | {:<6} | {:<18} | {:>7.1} | {:>10} | {:<6} | {:<38} |",
            pid,
            name,
            cpu_usage,
            mem,
            status,
            truncate(path.to_str().unwrap(), 38)
        );
    }

    println!(
        "  +{:-<6}+{:-<20}+{:-<9}+{:-<12}+{:-<8}+{:-<40}+",
        "", "", "", "", "", ""
    );

    // Summary bars (aggregate)
    let total_cpu = sys.global_cpu_usage();
    let total_mem = sys.used_memory() as f64 * 1024.0;
    let total_swap = sys.used_swap() as f64 * 1024.0;

    println!("Total CPU Usage: {:.1}%", total_cpu);
    println!("Total Memory Usage: {}", format_bytes(total_mem as u64));
    println!("Total Swap Usage: {}", format_bytes(total_swap as u64));
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}

fn format_bytes(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1000.0; // 1,048,576

    let b = bytes as f64;
    if b >= MB {
        format!("{:.2} MB", b / MB / 1000.0)
    } else if b >= KB {
        format!("{:.2} KB", b / KB)
    } else {
        format!("{} B", bytes)
    }
}
