use std::{thread, time};

use serde_json::from_str;
use sysinfo::System;
use tabled::{Table, Tabled};

#[derive(Debug, Tabled)]
struct TableInfo<'a> {
  name: &'a str,
  total: String,
  used: String,
  free: String,
}

impl<'a> TableInfo<'a> {
  fn new(name: &str, total: u64, used: u64) -> TableInfo {
    TableInfo {
      name,
      total: TableInfo::get_gb(total),
      used: TableInfo::get_gb(used),
      free: TableInfo::get_gb(total - used),
    }
  }

  fn get_gb(bytes: u64) -> String {
    format!("{}GB", bytes / 1024 / 1000 / 1000)
  }
}

fn main() {
  let mut sys = System::new_all();

  // let swap = TableInfo::new("Swap", sys.total_swap(), sys.used_swap());
  // let memory = TableInfo::new("Memory", sys.total_memory(), sys.used_memory());
  //
  // let table = Table::new([swap, memory]); // pass a slice or Vec of Tabled items
  // println!("{}", table);

  // CPU print information.

  loop {
    sys.refresh_all();
    print!("\x1B[2J\x1B[1;1H");

    println!("        -- CPU Usage --");

    // Adjust width of progress bar and columns
    let bar_width = 20;

    // print!("+{:-<10}+{:-<24}+{:-<9}+", "", "", "");
    print!("  +{:-<10}+{:-<24}+{:-<9}+\n", "", "", "");
    print!("  | {:<8} | {:<22} | {:>7} |\n", "CPU", "Usage", "%");
    print!("  +{:-<10}+{:-<24}+{:-<9}+\n", "", "", "");

    for (i, cpu) in sys.cpus().iter().enumerate() {
      let usage = cpu.cpu_usage();
      let bar = render_bar(usage, bar_width);
      print!(
        "  | {:<8} | {:<22} | {:>7.1} |\n",
        format!("Core {}", i),
        bar,
        usage
      );
    }

    print!("  +{:-<10}+{:-<24}+{:-<9}+\n", "", "", "");

    thread::sleep(time::Duration::from_millis(1000));
  }
}
fn render_bar(usage: f32, width: usize) -> String {
  let filled_len = ((usage / 100.0) * width as f32).round() as usize;
  let empty_len = width - filled_len;

  let filled = "▰".repeat(filled_len);
  let empty = "▱".repeat(empty_len);

  let color = usage_color(usage);
  let reset = "\x1b[0m";

  format!("{}[{}{}]{}", color, filled, empty, reset)
}
fn usage_color(usage: f32) -> &'static str {
  match usage {
    u if u >= 75.0 => "\x1b[31m", // Red
    u if u >= 50.0 => "\x1b[33m", // Yellow
    u if u >= 25.0 => "\x1b[32m", // Green
    _ => "\x1b[34m",              // Blue
  }
}
