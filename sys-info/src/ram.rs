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

pub fn get_memory_info(sys: &System) {
  let swap = TableInfo::new("Swap", sys.total_swap(), sys.used_swap());
  let memory = TableInfo::new("Memory", sys.total_memory(), sys.used_memory());

  let table = Table::new([swap, memory]).to_string().replace("\n", "\n  ");
  println!("  {}", table);
}
