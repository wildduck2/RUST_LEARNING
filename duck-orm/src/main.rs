fn main() {
  let file = match std::fs::read("index.ts") {
    Ok(file) => String::from_utf8_lossy(&file).to_string(),
    Err(err) => format!("Error reading file: {}", err),
  };

  println!("{:?}", file);
}
