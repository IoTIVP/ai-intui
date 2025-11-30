use std::{
    io,
    time::{Duration, Instant},
};

use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use humantime::format_duration;
use rand::{rngs::StdRng, Rng, SeedableRng};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    prelude::*,
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal,
};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Mode {
    AiObservability,
    Robotics,
    Cloud,
    DataForensics,
    Sandbox,
}

impl Mode {
    fn name(&self) -> &'static str {
        match self {
            Mode::AiObservability => "AI observability",
            Mode::Robotics => "Robotics",
            Mode::Cloud => "Cloud",
            Mode::DataForensics => "Data forensics",
            Mode::Sandbox => "Sandbox",
        }
    }

    fn short(&self) -> &'static str {
        match self {
            Mode::AiObservability => "AI",
            Mode::Robotics => "ROB",
            Mode::Cloud => "CLD",
            Mode::DataForensics => "DFX",
            Mode::Sandbox => "SBX",
        }
    }
}

#[derive(Clone, Copy)]
enum ColorProfile {
    Cyberpunk,
    Terminal,
}

impl Default for ColorProfile {
    fn default() -> Self {
        ColorProfile::Cyberpunk
    }
}

struct AppState {
    start_time: Instant,
    mode: Mode,
    color_profile: ColorProfile,
    logs: Vec<String>,
    cmd_input: String,
    cmd_active: bool,
    rng: StdRng,
}

impl AppState {
    fn new() -> Self {
        let mut logs = Vec::new();
        logs.push("ai-intui v0.9 — 1–5 to switch modes, : for command mode".into());
        logs.push("commands: help / ?, clear, set mode <ai|robotics|cloud|forensics|sandbox>".into());
        Self {
            start_time: Instant::now(),
            mode: Mode::AiObservability,
            color_profile: ColorProfile::Cyberpunk,
            logs,
            cmd_input: String::new(),
            cmd_active: false,
            rng: StdRng::from_entropy(),
        }
    }

    fn uptime(&self) -> Duration {
        self.start_time.elapsed()
    }

    fn push_log<S: Into<String>>(&mut self, line: S) {
        self.logs.push(line.into());
        if self.logs.len() > 512 {
            let drop = self.logs.len() - 512;
            self.logs.drain(0..drop);
        }
    }

    fn set_mode(&mut self, mode: Mode) {
        if self.mode != mode {
            self.mode = mode;
            self.push_log(format!("mode set → {}", self.mode.name()));
        }
    }

    fn tick(&mut self) {
        // Occasionally emit a synthetic log line depending on mode
        if self.rng.gen_bool(0.12) {
            let t = self.uptime().as_secs_f32();
            let msg = match self.mode {
                Mode::AiObservability => format!(
                    "AI[core] step={} temp={:.2} drift={:.3}",
                    (t * 12.0) as i32,
                    0.9 + 0.1 * (t * 0.3).sin(),
                    (t * 0.17).cos()
                ),
                Mode::Robotics => format!(
                    "ROB[path] jitter={:.1}ms torque={:.1}Nm",
                    4.0 + 3.0 * (t * 0.4).sin(),
                    18.0 + 2.0 * (t * 0.6).cos()
                ),
                Mode::Cloud => format!(
                    "CLD[node] p95={:.0}ms q_depth={:.2}",
                    210.0 + 85.0 * (t * 0.33).sin(),
                    0.4 + 0.3 * (t * 0.21).cos()
                ),
                Mode::DataForensics => format!(
                    "DFX[trace] anomalies={:.2} hash_shift={:.2}",
                    0.2 + 0.6 * (t * 0.27).sin().abs(),
                    0.1 + 0.4 * (t * 0.36).cos().abs()
                ),
                Mode::Sandbox => format!(
                    "SBX[synth] pattern={:.2} entropy={:.2}",
                    (t * 0.19).sin(),
                    (t * 0.23).cos().abs()
                ),
            };
            self.push_log(msg);
        }
    }

    fn process_command(&mut self) {
        let raw = self.cmd_input.trim().to_string();
        if raw.is_empty() {
            return;
        }

        // Echo command first
        self.push_log(format!(":> {}", raw));

        let lower = raw.to_ascii_lowercase();

        if lower == "help" || lower == "?" || lower == ":help" {
            self.push_log(
                "commands: \
set mode <ai|robotics|cloud|forensics|sandbox>, \
help / ?, clear",
            );
        } else if lower == "mode" || lower == ":mode" {
            self.push_log(format!("current mode → {}", self.mode.name()));
        } else if lower.starts_with("set mode ") || lower.starts_with(":set mode ") {
            let rest = lower
                .trim_start_matches(':')
                .trim_start_matches("set mode ")
                .trim();

            let target = match rest {
                "ai" | "ai-observability" => Some(Mode::AiObservability),
                "robotics" | "rob" => Some(Mode::Robotics),
                "cloud" | "cld" => Some(Mode::Cloud),
                "forensics" | "dfx" | "data" => Some(Mode::DataForensics),
                "sandbox" | "sbx" => Some(Mode::Sandbox),
                _ => None,
            };

            if let Some(m) = target {
                self.set_mode(m);
            } else {
                self.push_log("unknown mode. try: ai, robotics, cloud, forensics, sandbox");
            }
        } else if lower == "clear" || lower == ":clear" {
            self.logs.clear();
            self.push_log("logs cleared");
        } else {
            self.push_log("unrecognized command. type `help` or `?`");
        }

        self.cmd_input.clear();
    }
}

// Simple gradient bar: █ filled, space for empty
fn bar(norm: f32, len: usize) -> String {
    let n = norm.clamp(0.0, 1.0);
    let filled = (n * len as f32).round() as usize;
    let mut s = String::with_capacity(len);
    for i in 0..len {
        if i < filled {
            s.push('█');
        } else {
            s.push(' ');
        }
    }
    s
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = AppState::new();
    let tick_rate = Duration::from_millis(200);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_millis(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                // IMPORTANT: only act on actual key presses
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        // global exits (not in command mode)
                        KeyCode::Char('q') if !app.cmd_active => break,
                        KeyCode::Char('c')
                            if !app.cmd_active
                                && key.modifiers.contains(KeyModifiers::CONTROL) =>
                        {
                            break
                        }

                        // mode switching – must ALWAYS switch modes (even in cmd mode)
                        KeyCode::Char('1') => {
                            app.set_mode(Mode::AiObservability);
                            if app.cmd_active {
                                app.cmd_input.push('1');
                            }
                        }
                        KeyCode::Char('2') => {
                            app.set_mode(Mode::Robotics);
                            if app.cmd_active {
                                app.cmd_input.push('2');
                            }
                        }
                        KeyCode::Char('3') => {
                            app.set_mode(Mode::Cloud);
                            if app.cmd_active {
                                app.cmd_input.push('3');
                            }
                        }
                        KeyCode::Char('4') => {
                            app.set_mode(Mode::DataForensics);
                            if app.cmd_active {
                                app.cmd_input.push('4');
                            }
                        }
                        KeyCode::Char('5') => {
                            app.set_mode(Mode::Sandbox);
                            if app.cmd_active {
                                app.cmd_input.push('5');
                            }
                        }

                        // enter command mode with :
                        KeyCode::Char(':') => {
                            if app.cmd_active {
                                // already in command mode: treat ':' as input
                                app.cmd_input.push(':');
                            } else {
                                app.cmd_active = true;
                                app.cmd_input.clear();
                            }
                        }

                        // command-mode controls
                        KeyCode::Esc if app.cmd_active => {
                            app.cmd_input.clear();
                            app.cmd_active = false;
                        }
                        KeyCode::Enter if app.cmd_active => {
                            app.process_command();
                            app.cmd_active = false;
                        }
                        KeyCode::Backspace if app.cmd_active => {
                            app.cmd_input.pop();
                        }
                        KeyCode::Char(c) if app.cmd_active => {
                            // generic character input only in command mode
                            app.cmd_input.push(c);
                        }

                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.tick();
            last_tick = Instant::now();
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

fn ui(f: &mut Frame, app: &AppState) {
    let size = f.size();

    // Safety guard for tiny terminals (prevents ugly broken layouts)
    if size.width < 80 || size.height < 24 {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray))
            .title(Span::styled(
                "Ai-inTUI",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ));

        let msg = Paragraph::new("Ai-inTUI: terminal too small (min 80x24)")
            .alignment(Alignment::Center)
            .block(block);

        f.render_widget(msg, size);
        return;
    }

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // banner
            Constraint::Length(9), // metrics + system
            Constraint::Min(6),    // logs
            Constraint::Length(3), // command bar
        ])
        .split(size);

    draw_banner(f, rows[0], app);
    draw_metrics(f, rows[1], app);
    draw_logs(f, rows[2], app);
    draw_command(f, rows[3], app);
}

fn draw_banner(f: &mut Frame, area: Rect, app: &AppState) {
    // 25 / 50 / 25 so the center stays centered and uptime never pushes hints around
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(50),
            Constraint::Percentage(25),
        ])
        .split(area);

    // LEFT: stable [1–5] hints + : command
    let left = {
        let hint = "[1] AI  [2] ROB  [3] CLD  [4] DFX  [5] SBX  |  : command";
        Paragraph::new(hint)
            .alignment(Alignment::Left)
            .block(
                Block::default()
                    .borders(Borders::BOTTOM)
                    .border_style(Style::default().fg(Color::DarkGray)),
            )
    };

    // CENTER: Ai-inTUI + mode centered
    let center_line = Line::from(vec![
        Span::styled(
            "Ai-inTUI",
            Style::default()
                .fg(Color::LightCyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" • "),
        Span::styled(
            app.mode.name(),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
    ]);
    let mid = Paragraph::new(center_line)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
                .border_style(Style::default().fg(Color::DarkGray)),
        );

    // RIGHT: uptime only (no mode, so it never pushes center/hints)
    let right = {
        let uptime = format_duration(app.uptime()).to_string();
        let line = Line::from(vec![
            Span::styled("uptime ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                uptime,
                Style::default()
                    .fg(Color::LightBlue)
                    .add_modifier(Modifier::BOLD),
            ),
        ]);
        Paragraph::new(line)
            .alignment(Alignment::Right)
            .block(
                Block::default()
                    .borders(Borders::BOTTOM)
                    .border_style(Style::default().fg(Color::DarkGray)),
            )
    };

    f.render_widget(left, cols[0]);
    f.render_widget(mid, cols[1]);
    f.render_widget(right, cols[2]);
}

fn draw_metrics(f: &mut Frame, area: Rect, app: &AppState) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(area);

    draw_ai_metrics(f, cols[0], app);
    draw_system_panel(f, cols[1], app);
}

fn draw_ai_metrics(f: &mut Frame, area: Rect, app: &AppState) {
    let t = app.uptime().as_secs_f32();

    // Per-mode base shapes + light mode-specific accents via value ranges
    let (lat, gpu, tpm, err, q, jitter, trust) = match app.mode {
        Mode::AiObservability => (
            220.0 + 90.0 * (t * 0.33).sin(), // latency ms
            0.18 + 0.12 * (t * 0.27).cos(),  // service load
            13_000.0 + 5_000.0 * (t * 0.19).sin(),
            0.5 + 0.8 * (t * 0.41).sin().abs(),
            0.45 + 0.25 * (t * 0.23).cos(),
            7.0 + 3.0 * (t * 0.51).sin().abs(),
            0.92 - 0.08 * (t * 0.17).sin().abs(),
        ),
        Mode::Robotics => (
            80.0 + 40.0 * (t * 0.55).sin(),
            0.35 + 0.18 * (t * 0.37).cos(),
            4_800.0 + 1_800.0 * (t * 0.29).sin(),
            0.2 + 0.5 * (t * 0.63).sin().abs(),
            0.35 + 0.22 * (t * 0.33).cos(),
            4.0 + 2.5 * (t * 0.72).sin().abs(),
            0.89 - 0.10 * (t * 0.27).sin().abs(),
        ),
        Mode::Cloud => (
            260.0 + 110.0 * (t * 0.29).sin(),
            0.42 + 0.22 * (t * 0.31).cos(),
            19_000.0 + 7_000.0 * (t * 0.21).sin(),
            1.0 + 1.2 * (t * 0.45).sin().abs(),
            0.62 + 0.28 * (t * 0.26).cos(),
            5.5 + 3.5 * (t * 0.54).sin().abs(),
            0.87 - 0.12 * (t * 0.23).sin().abs(),
        ),
        Mode::DataForensics => (
            180.0 + 70.0 * (t * 0.39).sin(),
            0.24 + 0.15 * (t * 0.22).cos(),
            9_500.0 + 3_000.0 * (t * 0.18).sin(),
            0.3 + 0.9 * (t * 0.58).sin().abs(),
            0.28 + 0.18 * (t * 0.44).cos(),
            6.5 + 4.0 * (t * 0.63).sin().abs(),
            0.93 - 0.06 * (t * 0.31).sin().abs(),
        ),
        Mode::Sandbox => (
            150.0 + 120.0 * (t * 0.41).sin(),
            0.30 + 0.30 * (t * 0.36).cos(),
            7_000.0 + 9_000.0 * (t * 0.27).sin(),
            0.1 + 1.5 * (t * 0.49).sin().abs(),
            0.5 + 0.3 * (t * 0.38).cos(),
            8.0 + 5.0 * (t * 0.69).sin().abs(),
            0.80 - 0.18 * (t * 0.42).sin().abs(),
        ),
    };

    // Normalized for bars (keeps alignment)
    let lat_norm = (lat / 400.0).clamp(0.0, 1.0);
    let gpu_norm = gpu.clamp(0.0, 1.0);
    let tpm_norm = (tpm / 25_000.0).clamp(0.0, 1.0);
    let err_norm = (err / 3.0).clamp(0.0, 1.0);
    let q_norm = q.clamp(0.0, 1.0);
    let jitter_norm = (jitter / 20.0).clamp(0.0, 1.0);
    let trust_norm = trust.clamp(0.0, 1.0);

    let label_width = 15;
    let value_width = 8;
    let bar_len = 22;

    fn metric_line(
        label: &str,
        value: String,
        norm: f32,
        color: Color,
        label_width: usize,
        value_width: usize,
        bar_len: usize,
    ) -> Line<'static> {
        let mut lbl = label.to_string();
        if lbl.len() > label_width {
            lbl.truncate(label_width);
        }
        let label_padded = format!("{:label_width$}", lbl, label_width = label_width);
        let value_padded = format!("{:>value_width$}", value, value_width = value_width);
        let bar_str = bar(norm, bar_len);

        Line::from(vec![
            Span::styled(label_padded, Style::default().fg(Color::Gray)),
            Span::raw("  "),
            Span::styled(value_padded, Style::default().fg(Color::White)),
            Span::raw("  "),
            Span::styled(bar_str, Style::default().fg(color)),
        ])
    }

    // subtle accent: title color depends on mode, but same layout
    let title_color = match app.mode {
        Mode::AiObservability => Color::Cyan,
        Mode::Robotics => Color::LightYellow,
        Mode::Cloud => Color::LightMagenta,
        Mode::DataForensics => Color::LightGreen,
        Mode::Sandbox => Color::LightBlue,
    };

    let title = format!("AI metrics • {}", app.mode.name());

    let lines: Vec<Line> = vec![
        Line::from(""), // small padding
        metric_line(
            "latency p95",
            format!("{lat:.0} ms"),
            lat_norm,
            Color::LightGreen,
            label_width,
            value_width,
            bar_len,
        ),
        metric_line(
            "service load",
            format!("{:.0}%", gpu * 100.0),
            gpu_norm,
            Color::LightMagenta,
            label_width,
            value_width,
            bar_len,
        ),
        metric_line(
            "tokens/min",
            format!("{tpm:.0}"),
            tpm_norm,
            Color::Cyan,
            label_width,
            value_width,
            bar_len,
        ),
        metric_line(
            "errors/min",
            format!("{err:.2}"),
            err_norm,
            Color::Red,
            label_width,
            value_width,
            bar_len,
        ),
        metric_line(
            "queue depth",
            format!("{q:.2}"),
            q_norm,
            Color::Yellow,
            label_width,
            value_width,
            bar_len,
        ),
        metric_line(
            "sampler jitter",
            format!("{jitter:.1} ms"),
            jitter_norm,
            Color::LightBlue,
            label_width,
            value_width,
            bar_len,
        ),
        metric_line(
            "trust score",
            format!("{:.0}%", trust * 100.0),
            trust_norm,
            Color::Green,
            label_width,
            value_width,
            bar_len,
        ),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray))
        .title(Span::styled(
            title,
            Style::default()
                .fg(title_color)
                .add_modifier(Modifier::BOLD),
        ));

    let para = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false });

    f.render_widget(para, area);
}

fn draw_system_panel(f: &mut Frame, area: Rect, app: &AppState) {
    // Use app uptime so system panel "breathes" with the rest of the dashboard
    let t = app.uptime().as_secs_f32();

    let cpu = 0.40 + 0.25 * (t * 0.41).sin().abs();
    let mem = 0.55 + 0.20 * (t * 0.27).cos().abs();
    let disk = 0.30 + 0.35 * (t * 0.31).sin().abs();
    let net = 0.20 + 0.40 * (t * 0.22).cos().abs();

    let label_width = 12;
    let value_width = 6;
    let bar_len = 16;

    fn sys_line(
        label: &str,
        value: String,
        norm: f32,
        color: Color,
        label_width: usize,
        value_width: usize,
        bar_len: usize,
    ) -> Line<'static> {
        let mut lbl = label.to_string();
        if lbl.len() > label_width {
            lbl.truncate(label_width);
        }
        let label_padded = format!("{:label_width$}", lbl, label_width = label_width);
        let value_padded = format!("{:>value_width$}", value, value_width = value_width);
        let bar_str = bar(norm, bar_len);

        Line::from(vec![
            Span::styled(label_padded, Style::default().fg(Color::Gray)),
            Span::raw(" "),
            Span::styled(value_padded, Style::default().fg(Color::White)),
            Span::raw(" "),
            Span::styled(bar_str, Style::default().fg(color)),
        ])
    }

    let title = "system panel (fake data)";

    let lines: Vec<Line> = vec![
        Line::from(""),
        sys_line(
            "cpu load",
            format!("{:.0}%", cpu * 100.0),
            cpu,
            Color::LightGreen,
            label_width,
            value_width,
            bar_len,
        ),
        sys_line(
            "memory",
            format!("{:.0}%", mem * 100.0),
            mem,
            Color::LightMagenta,
            label_width,
            value_width,
            bar_len,
        ),
        sys_line(
            "disk io",
            format!("{:.0}%", disk * 100.0),
            disk,
            Color::Cyan,
            label_width,
            value_width,
            bar_len,
        ),
        sys_line(
            "net jitter",
            format!("{:.0}%", net * 100.0),
            net,
            Color::Yellow,
            label_width,
            value_width,
            bar_len,
        ),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray))
        .title(Span::styled(
            title,
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        ));

    let para = Paragraph::new(lines)
        .alignment(Alignment::Left)
        .block(block)
        .wrap(Wrap { trim: false });

    f.render_widget(para, area);
}

fn draw_logs(f: &mut Frame, area: Rect, app: &AppState) {
    let title = format!("logs • {}", app.mode.short());

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray))
        .title(Span::styled(
            title,
            Style::default()
                .fg(Color::LightBlue)
                .add_modifier(Modifier::BOLD),
        ));

    let inner = block.inner(area);

    // Build Line list and keep only last N that fit
    let mut lines: Vec<Line> = app
        .logs
        .iter()
        .map(|s| Line::from(s.clone()))
        .collect();

    let max_visible = inner.height.saturating_sub(1) as usize;
    if max_visible > 0 && lines.len() > max_visible {
        let start = lines.len() - max_visible;
        lines = lines[start..].to_vec();
    }

    let para = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false });

    f.render_widget(para, area);
}

fn draw_command(f: &mut Frame, area: Rect, app: &AppState) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray))
        .title(Span::styled(
            "command",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ));

    let line: Line = if app.cmd_active {
        // Active command mode: show prompt + current input
        let prompt = format!(":> {}", app.cmd_input);
        let hint = "  (help / ? / mode / set mode ai|robotics|cloud|forensics|sandbox • Esc to cancel)";
        Line::from(vec![
            Span::styled(prompt, Style::default().fg(Color::White)),
            Span::styled(hint, Style::default().fg(Color::DarkGray)),
        ])
    } else {
        // Idle: show a subtle hint, keep bar visible
        let hint = "press : for command mode • 1–5 to switch modes • q to quit";
        Line::from(vec![Span::styled(
            hint,
            Style::default().fg(Color::DarkGray),
        )])
    };

    let para = Paragraph::new(line)
        .block(block)
        .wrap(Wrap { trim: true });

    // render on full area so text is visible
    f.render_widget(para, area);
}
