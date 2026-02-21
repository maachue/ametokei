pub mod font;
pub mod path;
mod ser;

pub use ser::*;

/// minimal config
pub struct MinimalConfig {
    pub format_date: String,
    pub show_date: bool,
    pub utc: bool,
    pub hour12: bool,
    pub center: bool,
    pub sec: bool,
    pub font: String,
    pub tps: u8,
    pub fps: u8,
    pub padding: (u16, u16),
}
