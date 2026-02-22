use ratatui::layout::Rect;

use std::fmt::Display;

use chrono::{DateTime, TimeZone};

pub fn from_date_time<Tz: TimeZone>(time: DateTime<Tz>, format: &str) -> String
where
    Tz::Offset: Display,
{
    time.format(format).to_string()
}

pub struct DateState {
    pub area: Rect,
    pub spacing: u16, /* width only */
}
