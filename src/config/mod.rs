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
    // pub font: String, // actually useless (why do i need font name?)
    pub tps: u8,
    pub fps: u8,
    pub spacing: (u16, u16),
}
