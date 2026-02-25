use chrono::{DateTime, Local, TimeZone, Timelike, Utc};
use ratatui::layout::Rect;
use unicode_width::UnicodeWidthStr;

use super::{Direction, EachFrameImpl, Position, ShouldRender};
use crate::{
    config::{MeridiemConfig, RuntimeConfig},
    font::Font,
};

#[derive(Clone)]
pub struct Clock {
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
    pub date: Option<String>,
}
impl Clock {
    pub fn new(utc: bool, h12: bool, date: Option<&str>, meridiem: &MeridiemConfig) -> Self {
        if utc {
            let utc = Utc::now();

            Self::get(utc, h12, date, meridiem)
        } else {
            let local = Local::now();

            Self::get(local, h12, date, meridiem)
        }
    }

    fn get<Tz: TimeZone>(
        time: DateTime<Tz>,
        h12: bool,
        date: Option<&str>,
        meridiem: &MeridiemConfig,
    ) -> Self
    where
        Tz::Offset: std::fmt::Display,
    {
        let (is_pm, hours) = if h12 {
            let (is_pm, h12) = time.hour12();

            (Some(is_pm), h12 as u8)
        } else {
            (None, time.hour() as u8)
        };

        let mut date = date.map(|s| time.format(s).to_string());

        if let Some(date_str) = &mut date
            && let Some(is_pm) = is_pm
        {
            date_str.push_str(meridiem.get(is_pm));
        }

        Self {
            hours,
            minutes: time.minute() as u8,
            seconds: time.second() as u8,
            // is_pm,
            date,
        }
    }
}

#[derive(Clone, clap::ValueEnum)]
pub enum TimerMode {
    Dvd,
}
impl std::fmt::Display for TimerMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match *self {
            TimerMode::Dvd => "dvd",
        };

        s.fmt(f)
    }
}

#[derive(Copy, Clone)]
pub enum TimerRenderMode {
    Dvd(Direction),
}
impl Default for TimerRenderMode {
    fn default() -> Self {
        Self::Dvd(Direction::default())
    }
}
impl From<TimerMode> for TimerRenderMode {
    fn from(value: TimerMode) -> Self {
        match value {
            TimerMode::Dvd => TimerRenderMode::Dvd(Direction::default()),
        }
    }
}

pub struct ClockState {
    pub mode: Option<TimerRenderMode>,
    pub area: Rect,
    pub pos: Position,
    pub boundary: Rect,
    pub colon_show: bool,
    pub spacing: (u16, u16),
    pub show_seconds: bool,
}
impl ClockState {
    pub fn new(
        area: Rect,
        date_formatted: Option<&str>,
        config: &RuntimeConfig,
        font: &Font,
    ) -> color_eyre::Result<Self> {
        let boundary = area;
        let area = Self::get_area(area, date_formatted, config, font)?;

        Ok(Self {
            area,
            boundary,
            mode: config.timer_mode,
            pos: area.into(),
            colon_show: true,
            spacing: config.spacing,
            show_seconds: config.show_seconds,
        })
    }

    fn on_dvd_frame(&mut self) {
        let Some(TimerRenderMode::Dvd(dir)) = self.mode else {
            return;
        };

        let is_collision_horizontal = self.is_collision_horizontal();
        let is_collision_vertical = self.is_collision_vertical();

        let dir = if is_collision_vertical && is_collision_horizontal {
            dir.reflection_reverse()
        } else if is_collision_vertical {
            dir.reflection_vertical()
        } else if is_collision_horizontal {
            dir.reflection_horizontal()
        } else {
            dir
        };

        self.pos = self.pos.mv(dir);
        self.mode = Some(TimerRenderMode::Dvd(dir))
    }

    fn get_area(
        area: Rect,
        date_formatted: Option<&str>,
        config: &RuntimeConfig,
        font: &Font,
    ) -> color_eyre::Result<Rect> {
        let date_w = if let Some(date_formatted) = date_formatted {
            date_formatted.width() as u16
        } else {
            0
        };

        // 12:12:12
        // 12:12

        let timer_w = if config.show_seconds {
            (font.width * 6) + (font.colon_width * 2) + (config.spacing.0 * 7)
        } else {
            (font.width * 4) + font.colon_width + (config.spacing.0 * 4)
        };
        let timer_h = font.height + config.spacing.1;

        let clock_w = std::cmp::max(timer_w, date_w);
        let clock_h = if date_formatted.is_some() {
            timer_h + 1
        } else {
            timer_h
        };

        if area.width < clock_w || area.height < clock_h {
            return Err(color_eyre::eyre::eyre!(
                "terminal too small (expected: >= ({};{})).",
                clock_w,
                clock_h
            ));
        }

        let x = if config.center {
            area.width.saturating_sub(clock_w) / 2
        } else {
            0
        };
        let y = if config.center {
            area.height.saturating_sub(clock_h) / 2
        } else {
            0
        };

        Ok(Rect {
            x,
            y,
            width: clock_w,
            height: clock_h,
        })
    }

    fn get_area_with_pos(&self, pos: Position) -> Rect {
        pos.into_rect(self.area.width, self.area.height)
    }

    fn is_collision_vertical(&self) -> bool {
        self.pos.0 == 0 || (self.pos.0 + self.area.width) >= self.boundary.width
    }

    fn is_collision_horizontal(&self) -> bool {
        self.pos.1 == 0 || (self.pos.1 + self.area.height) >= self.boundary.height
    }

    fn handle_mode(&mut self, frame: u64) -> ShouldRender {
        if self.mode.is_none() {
            return ShouldRender::Skip;
        }

        if frame % 8 > 0 {
            return ShouldRender::Skip;
        }

        match self.mode.unwrap() {
            TimerRenderMode::Dvd(_) => self.on_dvd_frame(),
        }

        self.area = self.get_area_with_pos(self.pos);
        ShouldRender::Render
    }
}
impl EachFrameImpl for ClockState {
    fn on_frame(
        &mut self,
        _: &mut super::buffer::RenderBuffer,
        _: u64,
        frame: u64,
    ) -> ShouldRender {
        self.handle_mode(frame)
    }
}
