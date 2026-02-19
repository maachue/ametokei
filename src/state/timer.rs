use chrono::{DateTime, Local, Timelike, Utc};
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

#[derive(Clone)]
pub struct Timer {
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
    pub meridiem: Option<Meridiem>,
    pub date: String,
}
impl Timer {
    pub fn new(utc: bool, format12h: bool, format: &str) -> Self {
        if utc {
            let datetime = Utc::now();
            let (hours, minutes, seconds, meridiem) = Self::from_time_like(datetime, format12h);

            Self {
                hours,
                minutes,
                seconds,
                meridiem,
                date: format!("{}", datetime.format(format)),
            }
        } else {
            let datetime = Local::now();
            let (hours, minutes, seconds, meridiem) = Self::from_time_like(datetime, format12h);

            Self {
                hours,
                minutes,
                seconds,
                meridiem,
                date: format!("{}", datetime.format(format)),
            }
        }
    }

    fn from_time_like<T: Timelike>(time: T, format12h: bool) -> (u8, u8, u8, Option<Meridiem>) {
        let (hours, meridiem) = if format12h {
            let (is_pm, h12) = time.hour12();
            (h12 as u8, Some(Meridiem::from(is_pm)))
        } else {
            (time.hour() as u8, None)
        };

        (hours, time.minute() as u8, time.second() as u8, meridiem)
    }
}

pub struct TimerState {
    pub area: Rect,
    pub is_center: bool,
    pub show_sec: bool,
}
