pub mod date;
pub mod timer;

use chrono::{Local, Utc};
use color_eyre::{Result, Section};
use ratatui::layout::Rect;
use unicode_width::UnicodeWidthStr;

use self::{
    date::DateState,
    timer::{Timer, TimerState},
};
use crate::{
    config::{MeridiemConfig, MinimalConfig},
    font::Font,
};

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
    pub date: Option<String>,
    pub date_state: Option<DateState>,
    pub timer_state: TimerState,
    pub font: Font,

    // caching
    format_date: Option<String>,
}
impl State {
    pub fn new(size: Rect, config: &crate::config::MinimalConfig, font: Font) -> Result<Self> {
        let format_date = if config.show_date {
            Some(config.format_date.clone())
        } else {
            None
        };

        let (timer, maybe_date) = Self::get_time(
            config.utc,
            format_date.as_deref(),
            config.hour12,
            &config.mer,
        );
        let layouted = crate::widget::Layouted::get(
            size,
            config.spacing,
            config.center,
            config.sec,
            &font,
            maybe_date.as_deref(),
        )
        .with_suggestion(|| "Maybe make the terminal size a bit bigger?")?;

        let timer_state = TimerState {
            area: layouted.timer,
            spacing: config.spacing,
            show_sec: config.sec,
        };

        let date_state = if let Some(area) = layouted.date
            && let Some(date) = &maybe_date
        {
            Some(DateState {
                area,
                spacing: area.width.saturating_sub(date.width() as u16) / 2,
            })
        } else {
            None
        };

        Ok(Self {
            timer,
            date: maybe_date,
            timer_state,
            font,
            date_state,
            format_date,
        })
    }

    pub fn get_time(
        utc: bool,
        format: Option<&str>,
        hour12: bool,
        mer_conf: &MeridiemConfig,
    ) -> (Timer, Option<String>) {
        let (timer, mut date) = if utc {
            let utc = Utc::now();
            (
                Timer::from_time_like(utc, hour12),
                format.map(|t| utc.format(t).to_string()),
            )
        } else {
            let local = Local::now();

            (
                Timer::from_time_like(local, hour12),
                format.map(|t| local.format(t).to_string()),
            )
        };

        if let Some(mer) = &timer._meridiem
            && let Some(date) = date.as_mut()
        {
            date.push_str(&format!(" {}", mer.get(mer_conf)));
        }

        (timer, date)
    }

    pub fn on_resize(
        &mut self,
        width: u16,
        height: u16,
        config: &crate::config::MinimalConfig,
    ) -> Result<()> {
        let rect = Rect {
            x: 0,
            y: 0,
            width,
            height,
        };

        let layouted = crate::widget::Layouted::get(
            rect,
            config.spacing,
            config.center,
            config.sec,
            &self.font,
            self.date.as_deref(),
        )
        .with_suggestion(|| "Maybe make the terminal size a bit bigger?")?;

        self.timer_state.area = layouted.timer;

        if let Some(date) = layouted.date
            && let Some(date_state) = &mut self.date_state
        {
            date_state.area = date;
        }

        Ok(())
    }

    pub fn tick_timer(&mut self, config: &MinimalConfig) {
        (self.timer, self.date) = Self::get_time(
            config.utc,
            self.format_date.as_deref(),
            config.hour12,
            &config.mer,
        );
    }
}
