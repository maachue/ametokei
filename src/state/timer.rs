use chrono::{Local, Timelike, Utc};
use ratatui::layout::Rect;

#[derive(Copy, Clone)]
pub enum Meridiem {
    AM,
    PM,
}
impl From<bool> for Meridiem {
    fn from(is_pm: bool) -> Self {
        if is_pm { Self::PM } else { Self::AM }
    }
}
impl Meridiem {
    pub fn get_str(&self) -> &'static str {
        match self {
            Self::AM => "[AM]",
            Self::PM => "[PM]",
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
    pub fn new(utc: bool, format12h: bool) -> Self {
        if utc {
            Self::from_time_like(Utc::now(), format12h)
        } else {
            Self::from_time_like(Local::now(), format12h)
        }
    }

    fn from_time_like<T: Timelike>(time: T, format12h: bool) -> Self {
        let (hours, meridiem) = if format12h {
            let (is_pm, h12) = time.hour12();
            (h12 as u8, Some(Meridiem::from(is_pm)))
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
    pub is_center: bool,
    pub show_sec: bool,
}
