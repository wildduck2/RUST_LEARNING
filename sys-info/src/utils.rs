pub fn render_bar(usage: f32, width: usize) -> String {
  let filled_len = ((usage / 100.0) * width as f32).round() as usize;
  let empty_len = width - filled_len;

  let filled = "▰".repeat(filled_len);
  let empty = "▱".repeat(empty_len);

  let color = usage_color(usage);
  let reset = "\x1b[0m";

  format!("{}[{}{}]{}", color, filled, empty, reset)
}

pub fn usage_color(usage: f32) -> &'static str {
  match usage {
    u if u >= 75.0 => "\x1b[31m", // Red
    u if u >= 50.0 => "\x1b[33m", // Yellow
    u if u >= 25.0 => "\x1b[32m", // Green
    _ => "\x1b[34m",              // Blue
  }
}
