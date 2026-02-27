use std::collections::HashMap;

use std::path::Path;

use ratatui::style::Color;
use serde::{Deserialize, Serialize};

use crate::state::clock::TimerMode;

use super::MeridiemConfig;

#[derive(Default, Serialize, Deserialize)]
pub struct UserConfig {
    pub general: GeneralConfig,
    pub performance: PerformanceConfig,
    pub fontconfig: FontConfig,
    pub font: Option<HashMap<String, SerializeFont>>,
}
impl UserConfig {
    pub fn from_path(path: &Path) -> color_eyre::Result<Self> {
        let context = std::fs::read_to_string(path).map_err(|e| {
            color_eyre::eyre::eyre!("failed to read file: {} (due to: {e})", path.display())
        })?;
        match toml::from_str(&context) {
            Ok(res) => Ok(res),
            Err(e) => Err(color_eyre::Report::new(e)),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct GeneralConfig {
    pub show_seconds: bool,
    pub format_date: String,
    pub blink_colon: bool,
    pub color: Color,
    pub center: bool,
    pub hour12h: bool,
    pub utc: bool,
    pub meridiem: MeridiemConfig,
    pub hide_date: bool,
    pub timer_mode: Option<TimerMode>,
    pub font: String,
}
impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            show_seconds: false,
            format_date: "%Y-%m-%d".to_string(),
            blink_colon: false,
            center: false,
            color: Color::White,
            hide_date: false,
            hour12h: false,
            utc: false,
            meridiem: MeridiemConfig::default(),
            timer_mode: None,
            font: "tenki".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct PerformanceConfig {
    pub tps: u8,
    pub fps: u8,
}
impl Default for PerformanceConfig {
    fn default() -> Self {
        Self { tps: 60, fps: 60 }
    }
}

#[derive(Serialize, Deserialize)]
pub struct FontConfig {
    pub spacing_horizontal: u16,
    pub spacing_vertical: u16,
}
impl Default for FontConfig {
    fn default() -> Self {
        Self {
            spacing_vertical: 1,
            spacing_horizontal: 1,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerializeFont {
    pub colon_width: u16,
    pub num_width: u16,
    pub height: u16,
    // FIX: TOML not allowed the '\0'
    pub symbols: [char; 5],

    pub colon: Vec<u8>,
    pub zero: Vec<u8>,
    pub one: Vec<u8>,
    pub two: Vec<u8>,
    pub three: Vec<u8>,
    pub four: Vec<u8>,
    pub five: Vec<u8>,
    pub six: Vec<u8>,
    pub seven: Vec<u8>,
    pub eight: Vec<u8>,
    pub nine: Vec<u8>,
}
