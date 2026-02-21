pub mod timer;
pub mod date;

use chrono::{Local, Utc};
use color_eyre::Result;
use ratatui::layout::{Layout, Rect};

use self::{timer::{Timer, TimerState}, date::{DateState, Date}};
use crate::font::Font;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum ShouldRender {
    Render,
    Skip,
}
impl ShouldRender {
    #[allow(dead_code)]
    pub fn or(self, sr: Self) -> Self {
        match sr {
            Self::Render => self,
            Self::Skip => sr,
        }
    }
    pub fn is_render(&self) -> bool {
        *self == Self::Render
    }
}

pub struct State {
    pub timer: Timer,
    pub date: Option<DateState>,
    pub timer_state: TimerState,
    pub font: Font,
}
impl State {
    pub fn new(size: Rect, config: &crate::config::MinimalConfig, font: Font) -> Result<Self> {
        let (timer_area, date_area) = Self::get_area(size, config.padding, config.center, config.sec, !config.hide_date, &font)?;

        let (timer, maybe_date) = Self::get_time(config.utc, !config.hide_date, &config.format_date, config.hour12);

        let date = if let Some(area) = date_area && let Some(date) = maybe_date {
            Some(DateState {
                area,
                date,
            })
        } else {
            None
        };

        let timer_state = TimerState { area: timer_area };

        Ok( Self {
            timer,
            timer_state,
            font,
            date,
        }
        )
    }

    pub fn get_time(utc: bool, show_date: bool, format: &str, hour12: bool) -> (Timer, Option<Date>) {
        if utc {
            let utc = Utc::now();

            let date = if show_date { Some(Date::from_date_time(utc, format)) } else { None };

            (
                Timer::from_time_like(utc, hour12),
                date,
            )
        }  else {
            let local = Local::now();

            let date = if show_date { Some(Date::from_date_time(local, format)) } else { None };

            (
                Timer::from_time_like(local, hour12),
                date,
            )
        }
    }

    fn get_area(
        size: Rect,
        padding: (u16, u16),
        center: bool,
        show_sec: bool,
        show_date: bool,
        current_font: &Font,
    ) -> Result<(Rect, Option<Rect>)> {
        if current_font.width > size.width || current_font.height > size.height {
            return Err(color_eyre::eyre::eyre!(
                "The size of terminal too small (expected: > ({};{}) ; current: ({};{})",
                current_font.width,
                current_font.height,
                size.width,
                size.height,
            ));
        };

        let (timer_w, timer_h) = TimerState::get_size(padding, show_sec, current_font);
        let (date_w, date_h) = if show_date { (1, 1) } else { (0, 0) };

        let padding_w = if !center {
            0
        } else {
            size.width.saturating_sub(timer_w + date_w) / 2
        };
        let padding_h = if !center {
            0
        } else {
            size.height.saturating_sub(timer_h + date_h) / 2
        };

        let area = Rect {
            x: padding_w,
            y: padding_h,
            width: timer_w + date_w,
            height: timer_h + date_h,
        };

        if show_date {
            let [timer, date] = Layout::new(
                ratatui::layout::Direction::Vertical,
                ratatui::layout::Constraint::from_lengths([timer_h, date_h]),
            )
            .areas(area);

            Ok((timer, Some(date)))
        } else {
            Ok((area, None))
        }
    }
}
