pub mod timer;

use color_eyre::Result;

use crate::{
    font::Font,
    state::timer::{Timer, TimerState},
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
    pub timer_state: TimerState,
    pub font: Font,
}
