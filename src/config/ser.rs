use std::{collections::HashMap, path::Path};

use color_eyre::eyre::Result;
use ratatui::style::Color;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub general: GeneralConfig,
    pub performance: PerformanceConfig,
    pub fontconfig: FontConfig,
    pub fonts: Option<HashMap<String, SerializeFont>>,
}

#[derive(Serialize, Deserialize)]
pub struct GeneralConfig {
    pub format: String,
    pub color: Color,
    pub hide_date: bool,
    pub show_seconds: bool,
    pub utc: bool,
    pub format_12h: bool,
    pub center: bool,
    pub merdiem: MeridiemConfig,
    pub font: String,
}
impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            format: "%Y-%m-%d".to_string(),
            hide_date: false,
            color: Color::White,
            show_seconds: false,
            utc: false,
            format_12h: false,
            center: true,
            merdiem: MeridiemConfig::default(),
            font: "digital".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MeridiemConfig {
    pub am: String,
    pub pm: String,
}
impl Default for MeridiemConfig {
    fn default() -> Self {
        Self {
            am: "[AM]".to_string(),
            pm: "[PM]".to_string(),
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
    pub spacing_width_between_digits: u16,
    pub spacing_between_timer_and_date: u16,
}

#[derive(Serialize, Deserialize)]
pub struct SerializeFont {
    pub width_number: u16, // user must be report this (for easier when parsing & checking)
    pub width_colon: u16,  // user must be report this (for easier when parsing & checking)
    pub height: u16,       // user must be report this (for easier when parsing & checking)
    pub zero: Vec<String>,
    pub one: Vec<String>,
    pub two: Vec<String>,
    pub three: Vec<String>,
    pub four: Vec<String>,
    pub five: Vec<String>,
    pub six: Vec<String>,
    pub seven: Vec<String>,
    pub eight: Vec<String>,
    pub nine: Vec<String>,

    pub colon: Vec<String>,
}

impl Config {
    pub fn from_path(p: &Path) -> Result<Self> {
        let context = std::fs::read_to_string(p).map_err(|e| {
            color_eyre::eyre::eyre!("failed to read file: {} (due to: {e})", p.display())
        })?;
        match toml::from_str(&context) {
            Ok(res) => Ok(res),
            Err(e) => Err(color_eyre::Report::new(e)),
        }
    }
}
