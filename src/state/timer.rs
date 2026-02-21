use std::fmt::Display;

use chrono::{DateTime, Offset, TimeZone, Timelike};
use color_eyre::eyre::Result;
use ratatui::layout::Rect;

use crate::font::Font;

#[derive(Copy, Clone)]
pub enum Meridiem {
    AM,
    PM,
}
impl From<bool> for Meridiem {
    fn from(value: bool) -> Self {
        match value {
            true => Self::PM,
            false => Self::AM,
        }
    }
}
impl Meridiem {
    pub fn get(self, config: &crate::config::MeridiemConfig) -> &str {
        match self {
            Self::AM => &config.am,
            Self::PM => &config.pm,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Timer {
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
    pub meridiem: Option<Meridiem>,
}
impl Timer {
    pub fn from_time_like<T: Timelike>(time: T, hour12: bool) -> Self {
        let (hours, meridiem) = if hour12 {
            let (is_pm, h) = time.hour12();

            (h as u8, Some(is_pm.into()))
        } else {
            (time.hour() as u8, None)
        };

        Self {
            hours,
            minutes: time.minute() as u8,
            seconds: time.second() as u8,
            meridiem,
        }
    }
}

pub struct TimerState {
    pub area: Rect,
    pub padding: (u16, u16),
    pub show_sec: bool,
}
impl TimerState {
    pub fn get_size(
        padding: (u16, u16),
        show_sec: bool,
        current_font: &Font,
    ) -> (u16 /* width */, u16 /* height */) {
        let digit = if show_sec { 6 } else { 4 };
        let colons = if show_sec { 2 } else { 1 };

        // 12:12:12
        // 12:12

        let need_padding = if show_sec { 7 } else { 1 };

        (
            (current_font.width * digit)
                + (current_font.colon.width * colons)
                + (need_padding * padding.0),
            current_font.height + (padding.1 * 2),
        )
    }
}
