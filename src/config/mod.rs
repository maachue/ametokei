use ratatui::style::Color;

pub mod meridiem;

use crate::state::clock::TimerRenderMode;

pub use self::meridiem::MeridiemConfig;

pub struct RuntimeConfig {
    pub show_seconds: bool,
    pub format_date: Option<String>,
    pub blink_colon: bool,
    pub color: Color,
    pub fps: u8,
    pub center: bool,
    pub tps: u8,
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
