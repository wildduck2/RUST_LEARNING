# ✅ TODO: Build a System Resource Monitor in Rust

## 📦 Project Setup
- [ ] Create a new Rust project using `cargo new sys_monitor`
- [ ] Add dependencies in `Cargo.toml`:
  - [ ] `sysinfo` for accessing system metrics
  - [ ] `crossterm` for terminal control and formatting (optional)

## 🧠 Core Functionality
### 🔁 System Initialization
- [ ] Initialize the `System` object from `sysinfo`
- [ ] Implement periodic system info refreshing (e.g. every second)

### 📊 Display System Metrics
- [ ] Fetch and display overall CPU usage
- [ ] Fetch and display total and used memory (RAM)
- [ ] Fetch and display disk usage (used vs. total space)
- [ ] Optionally display per-core CPU usage

### 🎨 Terminal UI (Optional but recommended)
- [ ] Clear and redraw the terminal each cycle
- [ ] Format output nicely using `crossterm`
- [ ] Highlight metrics with simple visual indicators (e.g., percentage bars)

## 🧪 Testing & Debugging
- [ ] Manually verify that values update in real time
- [ ] Test app on different OS environments (Linux, macOS, Windows)

## 🧹 Optional Enhancements
- [ ] Add keyboard input to quit (e.g., press `q` to exit)
- [ ] Add color coding for load thresholds (e.g., red for high CPU usage)
- [ ] Add CLI flags to show/hide certain metrics
- [ ] Add logging or snapshot feature (e.g., save usage data to a file)

## 🚀 Final Steps
- [ ] Review and refactor code for clarity and performance
- [ ] Write a README.md with usage instructions
- [ ] Package and release binary if desired


## CPU Monitor (htop-style)

- [ ] Refresh system stats in a loop (every 1s)
- [ ] Clear terminal before each redraw
- [ ] Iterate over `sys.cpus()` to get per-core usage
- [ ] Format each CPU as:
      Core N [██████████------] XX.X%
- [ ] Pad output for alignment
- [ ] Optionally add coloring (via `crossterm`)

