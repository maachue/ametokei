use chrono::{Local, MIN_DATETIME, Timelike, Utc};
use color_eyre::eyre::Result;
use ratatui::layout::Rect;

use crate::font::Font;

#[derive(Clone)]
pub struct Timer {
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
}
impl Timer {
    pub fn new(utc: bool) -> Self {
        if utc {
            Self::from_time_like(Utc::now())
        } else {
            Self::from_time_like(Local::now())
        }
    }

    fn from_time_like<T: Timelike>(time: T) -> Self {
        Self {
            hours: time.hour() as u8,
            minutes: time.minute() as u8,
            seconds: time.second() as u8,
        }
    }
}

pub struct TimerState {
    pub area: Rect,
    pub center: bool,
    pub show_sec: bool,
    pub padding: (u16 /* width */, u16 /* height */),
}
impl TimerState {
    pub fn new(
        area: Rect,
        padding: (u16, u16),
        show_sec: bool,
        center: bool,
        current_font: &Font,
    ) -> Result<Self> {
        let area = Self::get_area(area, padding, show_sec, center, current_font)?;

        Ok(Self {
            area,
            center,
            show_sec,
            padding,
        })
    }

    pub fn get_area(
        area: Rect,
        padding: (u16, u16),
        show_sec: bool,
        center: bool,
        current_font: &Font,
    ) -> Result<Rect> {
        if current_font.width > area.width || current_font.height > area.height {
            return Err(color_eyre::eyre::eyre!(
                "The size of terminal too small (expected: > ({};{}) ; current: ({};{})",
                current_font.width,
                current_font.height,
                area.width,
                area.height,
            ));
        }

        let digit: u16 = if show_sec { 6 } else { 4 };
        let colons: u16 = if show_sec { 2 } else { 1 };

        // 12:12:12
        // 12:12

        let need_padding = if show_sec { 7 } else { 5 };

        let width = (current_font.width * digit)
            + (current_font.colon.width * colons)
            + (need_padding * padding.0);
        let height = current_font.height + (current_font.height * 2);

        let padding_w = if center {
            area.width.saturating_sub(width) / 2
        } else {
            0
        };
        let padding_h = if center {
            area.height.saturating_sub(height) / 2
        } else {
            0
        };

        Ok(Rect {
            x: padding_w,
            y: padding_h,
            width,
            height,
        })
    }
}
