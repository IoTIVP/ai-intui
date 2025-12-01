<p align="center">
  <img src="https://img.shields.io/badge/rust-stable-orange?style=flat-square" alt="Rust Stable">
  <img src="https://img.shields.io/github/actions/workflow/status/IoTIVP/ai-intui/ci.yml?style=flat-square&label=CI" alt="CI Status">
  <img src="https://img.shields.io/badge/license-MIT-green.svg?style=flat-square" alt="License MIT">
  <img src="https://img.shields.io/badge/version-0.9.0-blue?style=flat-square" alt="Version">
</p>

---
![Ai-inTUI demo](assets/ai-intui-demo.png)


# Ai-inTUI

Ai-inTUI is a **Rust terminal dashboard** (Crossterm + Ratatui) for simulating
AI observability, robotics, cloud, and forensics telemetry — all from a clean,
minimal TUI.

No Textual. No BubbleTea. Just Rust, Crossterm, and Ratatui.

## Features

- Top banner with:
  - Ai-inTUI title
  - Current mode
  - Uptime
  - Stable mode hints `[1] AI [2] ROB [3] CLD [4] DFX [5] SBX | : command`
- AI metrics panel:
  - latency p95
  - service load
  - tokens/min
  - errors/min
  - queue depth
  - sampler jitter
  - trust score
- System panel (fake data):
  - CPU load
  - Memory
  - Disk I/O
  - Net jitter
- Logs panel with synthetic events and auto-scrolling
- Command bar at the bottom (`:>` style) with a mini command language

## Controls

- `1` – AI observability mode
- `2` – Robotics mode
- `3` – Cloud mode
- `4` – Data forensics mode
- `5` – Sandbox mode
- `:` – Enter command mode
- `Esc` – Cancel command mode
- `q` – Quit (when not in command mode)
- `Ctrl+C` – Quit (when not in command mode)

### Commands

Type these after pressing `:`:

- `help` or `?` – Show help in the log panel
- `mode` – Show the current mode
- `set mode ai|robotics|cloud|forensics|sandbox` – Switch mode
- `clear` – Clear the logs

## Install & Run

```bash
git clone https://github.com/IoTIVP/ai-intui.git
cd ai-intui

cargo run

Requires Rust 1.75+ (stable) and a terminal at least 80x24.


## Roadmap

### v0.9.x (Current Series)
- Improve metrics rendering and visual consistency
- Fine-tune layout spacing and borders
- Add more realistic synthetic telemetry (less random)
- Enhance command system (aliases, history)
- Add optional color themes (cyberpunk / terminal)
- Prepare GIF demo for the README

### v1.0 Goals
- Help overlay (instead of log-only help)
- Configurable metrics update rates
- Optional real data feeds
- Windows/macOS/Linux binaries via GitHub Releases
- Full crates.io publishing
- Screenshot + GIF showcase
- Plugin-style mode extensions
