use std::{cell::RefCell, rc::Rc};

use color_eyre::eyre::Result;
use ratatui::layout::Rect;
use tinyvec::ArrayVec;

pub mod buffer;
pub mod clock;

use crate::{config::RuntimeConfig, font::Font};

use self::{
    buffer::RenderBuffer,
    clock::{Clock, ClockState},
};

pub type Cell = ArrayVec<[CellType; 3]>;
pub type Column = Rc<RefCell<Vec<Cell>>>;

#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub enum CellType {
    Fast,
    Normal,
    Slow,
    Tail,
    #[default]
    None,
}
impl CellType {
    pub fn is_dropping_cell(&self) -> bool {
        use CellType::*;
        matches!(*self, Fast | Normal | Slow)
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ShouldRender {
    Render,
    Skip,
}
impl ShouldRender {
    pub fn or(self, sr: Self) -> Self {
        match sr {
            Self::Skip => self,
            Self::Render => sr,
        }
    }

    pub fn is_render(&self) -> bool {
        *self == Self::Render
    }
}

pub trait EachFrameImpl {
    fn on_frame(&mut self, _: &mut RenderBuffer, _: u64, _: u64) -> ShouldRender {
        ShouldRender::Skip
    }
}

#[derive(Copy, Clone, Default)]
pub enum Direction {
    #[default]
    LeftTop,
    LeftBottom,
    RightTop,
    RightBottom,
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    pub fn reflection_horizontal(&self) -> Self {
        use Direction::*;
        match *self {
            LeftTop => LeftBottom,
            LeftBottom => LeftTop,
            RightTop => RightBottom,
            RightBottom => RightTop,
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }

    pub fn reflection_vertical(&self) -> Self {
        use Direction::*;
        match *self {
            LeftTop => RightTop,
            LeftBottom => RightBottom,
            RightTop => LeftTop,
            RightBottom => LeftBottom,
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }

    pub fn reflection_reverse(&self) -> Self {
        use Direction::*;
        match *self {
            LeftTop => RightBottom,
            LeftBottom => RightTop,
            RightTop => LeftBottom,
            RightBottom => LeftTop,
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Position(u16, u16);
impl From<Rect> for Position {
    fn from(value: Rect) -> Self {
        Self(value.x, value.y)
    }
}
impl Position {
    pub fn into_rect(self, width: u16, height: u16) -> Rect {
        Rect {
            height,
            width,
            x: self.0,
            y: self.1,
        }
    }

    pub fn mv(self, dir: Direction) -> Self {
        use Direction::*;
        let Position(x, y) = self;
        match dir {
            LeftTop => Self(x.saturating_sub(1), y.saturating_sub(1)),
            LeftBottom => Self(x.saturating_sub(1), y.saturating_add(1)),
            RightTop => Self(x.saturating_add(1), y.saturating_sub(1)),
            RightBottom => Self(x.saturating_add(1), y.saturating_add(1)),
            Up => Self(x, y.saturating_sub(1)),
            Down => Self(x, y.saturating_add(1)),
            Left => Self(x.saturating_sub(1), y),
            Right => Self(x.saturating_add(1), y),
        }
    }
}

#[derive(Copy, Clone, Default, clap::ValueEnum, PartialEq, Eq)]
pub enum Mode {
    #[default]
    Rain,
    Snow,
    Meteor,
    Disable,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match *self {
            Mode::Rain => "rain",
            Mode::Snow => "snow",
            Mode::Meteor => "meteor",
            Mode::Disable => "disable",
        };

        s.fmt(f)
    }
}
impl Mode {
    pub fn get_frame_by_speed(&self, s: CellType) -> u64 {
        use CellType::*;
        use Mode::*;

        match self {
            Rain => match s {
                Fast => 1,
                Normal => 2,
                Slow => 3,
                _ => 0,
            },
            Snow => match s {
                Fast => 2,
                Normal => 4,
                Slow => 6,
                _ => 0,
            },
            Meteor => 4,
            _ => 0,
        }
    }
}

pub struct State {
    pub rb: RenderBuffer,
    pub clock: Clock,
    pub clock_state: ClockState,
    // pub weather: T,
    frame: u64,
    // rng: SmallRng,
    seed: u64,
    pub font: Font,
}
impl State {
    pub fn new(size: Rect, config: &RuntimeConfig, font: Font) -> Result<Self> {
        let clock = Clock::new(
            config.utc,
            config.hour12h,
            config.format_date.as_deref(),
            &config.meridiem,
        );

        let clock_state = ClockState::new(size, clock.date.as_deref(), config, &font)?;

        Ok(Self {
            rb: RenderBuffer::new(size),
            frame: 0,
            clock,
            clock_state,
            seed: 0,
            font,
        })
    }

    pub fn tick_timer(&mut self, config: &RuntimeConfig) {
        self.clock = Clock::new(
            config.utc,
            config.hour12h,
            config.format_date.as_deref(),
            &config.meridiem,
        );

        if config.blink_colon {
            self.clock_state.colon_show = !self.clock_state.colon_show;
        }
    }

    pub fn on_resize(&mut self, width: u16, height: u16, config: &RuntimeConfig) -> Result<()> {
        let area = Rect {
            x: 0,
            y: 0,
            width,
            height,
        };

        self.clock_state = ClockState::new(area, self.clock.date.as_deref(), config, &self.font)?;

        Ok(())
    }

    pub fn tick(&mut self) -> ShouldRender {
        self.frame = if self.frame == u64::MAX { 0 } else { self.frame.saturating_add(1) };
        // self.seed = self.rng.next_u64();

        // self.weather.on_frame(&mut self.rb, self.seed, self.frame)
            self.clock_state.on_frame(&mut self.rb, self.seed, self.frame)
    }
}
