use crate::utils;
use sysinfo::System;

pub fn get_cpu_info(sys: &System) {
  // Adjust width of progress bar and columns
  let bar_width = 20;

  // print!("+{:-<10}+{:-<24}+{:-<9}+", "", "", "");
  print!("  +{:-<10}+{:-<24}+{:-<9}+\n", "", "", "");
  print!("  | {:<8} | {:<22} | {:>7} |\n", "CPU", "Usage", "%");
  print!("  +{:-<10}+{:-<24}+{:-<9}+\n", "", "", "");

  for (i, cpu) in sys.cpus().iter().enumerate() {
    let usage = cpu.cpu_usage();
    let bar = utils::render_bar(usage, bar_width);
    print!(
      "  | {:<8} | {:<22} | {:>7.1} |\n",
      format!("Core {}", i),
      bar,
      usage
    );
  }

  print!("  +{:-<10}+{:-<24}+{:-<9}+\n", "", "", "");
}
