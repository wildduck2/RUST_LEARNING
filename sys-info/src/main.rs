mod cpu;
mod process;
mod ram;
mod utils;

use core::time;
use std::thread;

use sysinfo::{Pid, System};

fn main() {
  let mut sys = System::new_all();

  loop {
    sys.refresh_all();
    // Clear the screen
    print!("\x1B[2J\x1B[1;1H");

    // CPU print information.
    cpu::get_cpu_info(&sys);
    // RAM print information.
    ram::get_memory_info(&sys);

    process::get_process_info(&sys);

    // Sleep the thread for a bit.
    thread::sleep(time::Duration::from_millis(1000));
  }
}

pub fn get_full_info(sys: &System) -> () {
  let pid = std::process::id() as i32;

  if let Some(process) = sys.process(Pid::from(pid as usize)) {
    let mem_mb = process.memory() as f64 / 1024.0 / 1000.0;
    let virt_mb = process.virtual_memory() as f64 / 1024.0 / 1000.0;
    let cpu = process.cpu_usage();

    println!("Memory usage      : {:.2} MB", mem_mb);
    println!("Virtual memory    : {:.2} MB", virt_mb);
    println!("CPU usage         : {:.2}%", cpu);
  } else {
    println!("Process info not found.");
  }
}
