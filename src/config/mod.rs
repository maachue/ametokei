pub mod font;
pub mod handle;
mod ser;

use ratatui::style::Color;
pub use ser::*;

/// minimal config
pub struct MinimalConfig {
    pub format_date: String,
    pub show_date: bool,
    pub utc: bool,
    pub hour12: bool,
    pub center: bool,
    pub sec: bool,
    pub tps: u8,
    pub fps: u8,
    pub spacing: (u16, u16),
    pub color: Color,
    pub mer: MeridiemConfig,
}
impl Default for MinimalConfig {
    fn default() -> Self {
        Self {
            format_date: "%Y-%m-%d".to_string(),
            show_date: true,
            utc: false,
            hour12: false,
            center: false,
            sec: false,
            tps: 60,
            fps: 60,
            spacing: (1, 1),
            color: Color::White,
            mer: MeridiemConfig::default(),
        }
    }
}
impl MinimalConfig {
    pub fn cmd(&mut self, cmd: &crate::cli::Cli) {
        if let Some(hide_date) = cmd.hide_date {
            self.show_date = !hide_date
        }

        if let Some(hour12) = cmd.hour12 {
            self.hour12 = hour12
        }

        if let Some(center) = cmd.center {
            self.center = center
        }

        if let Some(tps) = cmd.tps {
            self.tps = tps
        }

        if let Some(sec) = cmd.show_seconds {
            self.sec = sec
        }

        if let Some(fps) = cmd.fps {
            self.fps = fps
        }

        if let Some(color) = cmd.timer_color {
            self.color = color
        }

        if let Some(date) = &cmd.date {
            self.format_date = date.to_string()
        }
    }
}
