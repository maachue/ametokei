use ratatui::style::Color;

pub mod meridiem;

use crate::{cli::Cli, state::clock::TimerRenderMode};

pub use self::meridiem::MeridiemConfig;

pub struct RuntimeConfig {
    pub show_seconds: bool,
    pub format_date: Option<String>,
    pub blink_colon: bool,
    pub color: Color,
    pub fps: u8,
    pub tps: u8,
    pub center: bool,
    pub hour12h: bool,
    pub utc: bool,
    pub meridiem: MeridiemConfig,
    pub spacing: (u16, u16),
    pub timer_mode: Option<TimerRenderMode>,
}
impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            show_seconds: false,
            format_date: Some("%Y-%m-%d".to_string()),
            blink_colon: false,
            center: false,
            color: Color::White,
            fps: 60,
            tps: 60,
            hour12h: false,
            utc: false,
            meridiem: MeridiemConfig::default(),
            spacing: (1, 1),
            timer_mode: None,
        }
    }
}
impl RuntimeConfig {
    pub fn cli_override(&mut self, cli: Cli) {
        self.fps = cli.fps.unwrap_or(self.fps);
        self.tps = cli.tps.unwrap_or(self.tps);
        self.hour12h = cli.hour12.unwrap_or(self.hour12h);
        self.center = cli.center.unwrap_or(self.center);
        self.utc = cli.utc.unwrap_or(self.utc);
        self.show_seconds = cli.show_seconds.unwrap_or(self.show_seconds);
        self.blink_colon = cli.blink_colon.unwrap_or(self.blink_colon);

        if let Some(format_date) = cli.format_date {
            self.format_date = Some(format_date);
        }

        if cli.hide_date {
            self.format_date = None;
        }

        self.color = cli.timer_color.unwrap_or(self.color);

        if let Some(timer_mode) = cli.timer_mode {
            self.timer_mode = Some(timer_mode.into());
        }
    }
}
