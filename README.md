<p align="center">
  <img src="https://img.shields.io/badge/version-v0.9.0-blue?style=for-the-badge" />
  <img src="https://img.shields.io/badge/built_with-Rust-orange?style=for-the-badge&logo=rust" />
  <img src="https://img.shields.io/badge/terminal-Ratatui-4B8BBE?style=for-the-badge" />
  <img src="https://img.shields.io/badge/backend-Crossterm-6E6E6E?style=for-the-badge" />
  <img src="https://img.shields.io/badge/license-MIT-green?style=for-the-badge" />
</p>


# Ai-inTUI

**Ai-inTUI** is a Rust + Ratatui terminal dashboard for AI, robotics, cloud, and data forensics observability — inspired by Textual/Bubble Tea–style apps but built with **Crossterm + Ratatui only**.

It gives you a clean terminal UI with:

- Top banner with **Ai-inTUI**, current **mode**, and **uptime**
- AI metrics panel (latency, service load, tokens/min, errors/min, queue depth, sampler jitter, trust score)
- System panel (fake CPU, memory, disk I/O, net jitter)
- Scrolling logs
- Command bar with `:>` style commands

---

## Features

- **Modes** (hotkeys `1–5`):
  - `AI observability`
  - `Robotics`
  - `Cloud`
  - `Data forensics`
  - `Sandbox`

- **Panels**
  - **Banner**:  
    - Left: `[1] AI  [2] ROB  [3] CLD  [4] DFX  [5] SBX  |  : command`  
    - Center: `Ai-inTUI • <mode>` (centered)  
    - Right: `uptime <HH:MM:SS>`

  - **AI Metrics panel**  
    Clean, aligned horizontal bar metrics (no braille noise), including:
    - latency p95
    - service load
    - tokens/min
    - errors/min
    - queue depth
    - sampler jitter
    - trust score

  - **System panel (fake data)**  
    - cpu load
    - memory
    - disk io
    - net jitter

  - **Logs panel**
    - Synthetic events based on active mode
    - Logs scroll as they grow (oldest dropped, newest kept)
    - Commands echoed as `:> your_command`

  - **Command bar**
    - Always visible at the bottom
    - `:>` prompt when in command mode
    - Subtle hints when idle

---

## Controls

### Keys

```text
1–5        Switch mode (AI / Robotics / Cloud / Forensics / Sandbox)
:          Enter command mode
Esc        Cancel command mode
Enter      Run command (when in command mode)
q          Quit (when not in command mode)
Ctrl+C     Quit
