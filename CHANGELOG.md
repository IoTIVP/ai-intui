# Changelog

All notable changes to **Ai-inTUI** will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [0.9.0] - 2025-11-30

### Added
- Core Ai-inTUI dashboard using **Rust + Crossterm + Ratatui** only.
- Top banner with:
  - Left: stable mode hints `[1] AI [2] ROB [3] CLD [4] DFX [5] SBX | : command`
  - Center: `Ai-inTUI • <mode>`
  - Right: human-readable uptime.
- Five modes:
  - `AI observability`
  - `Robotics`
  - `Cloud`
  - `Data forensics`
  - `Sandbox`
- AI metrics panel with aligned bar-graph metrics:
  - `latency p95`
  - `service load`
  - `tokens/min`
  - `errors/min`
  - `queue depth`
  - `sampler jitter`
  - `trust score`
- System panel (fake data) with CPU, memory, disk I/O, and net jitter.

### Commands
- `:` enters command mode.
- `help` / `?` show inline help in the log pane.
- `set mode <ai|robotics|cloud|forensics|sandbox>` switches modes and logs the change.
- `mode` shows the current mode.
- `clear` wipes the log buffer.

### Interaction
- `1–5` always switch modes (even while typing a command).
- `q` exits (when not in command mode).
- `Ctrl+C` exits (when not in command mode).
- `Esc` cancels command mode.

### Fixed / Non-negotiables
- Command input never double-types characters.
- Command bar is always visible and not cut off.
- Logs scroll smoothly; oldest entries are dropped (max 512).
- Metric labels and bars are aligned in clean columns.
- `sampler jitter` bar is horizontally aligned with other metrics.

### Internal
- Added GitHub Actions workflow for `cargo build`, `cargo fmt`, and `cargo clippy -- -D warnings`.
