use std::fmt::Display;

use chrono::{DateTime, TimeZone};
use ratatui::layout::Rect;

use crate::state::timer::Meridiem;

#[derive(Clone)]
pub struct Date(String);
impl Date {
    pub fn from_date_time<Tz: TimeZone>(time: DateTime<Tz>, format: &str) -> Self
    where
        Tz::Offset: Display,
    {
        Self(time.format(format).to_string())
    }
    pub fn update_with_meridiem(
        &mut self,
        meridiem: Meridiem,
        config: &crate::config::MeridiemConfig,
    ) {
        self.0.push_str(meridiem.get(config))
    }
}

pub struct DateState {
    pub area: Rect,
    pub date: Date,
}