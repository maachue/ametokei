use crate::{
    config::WeatherInfomation,
    state::{
        EachFrameImpl, Mode, ShouldRender, buffer::RenderBuffer, dropping::DroppingState,
        tail::TailState, wind::WindState,
    },
    widget::{AsWeatherWidget, weather::GeneralWeatherWidget},
};

use super::WeatherImpl;

const DEF_LEVEL: u16 = 50;
const DEF_TAIL_LEVEL: u16 = 500;

pub struct GeneralDropping {
    wind: WindState,
    dropping: DroppingState,
}

impl GeneralDropping {
    pub fn new(cfg: WeatherInfomation) -> Self {
        Self {
            wind: WindState::new(cfg.wind),
            dropping: DroppingState {
                threshold: cfg.level.unwrap_or(DEF_LEVEL),
                mode: cfg.mode,
            },
        }
    }
}

impl WeatherImpl for GeneralDropping {}

impl EachFrameImpl for GeneralDropping {
    fn on_frame(&mut self, rb: &mut RenderBuffer, seed: u64, frame: u64) -> ShouldRender {
        self.wind
            .on_frame(rb, seed, frame)
            .or(self.dropping.on_frame(rb, seed, frame))
    }
}

impl AsWeatherWidget for GeneralDropping {
    type Weather = GeneralWeatherWidget;

    fn as_weather_widget(&self) -> Self::Weather {
        use Mode::*;
        match self.dropping.mode {
            Rain => GeneralWeatherWidget::Rain(self.wind.direction),
            Snow => GeneralWeatherWidget::Snow,
            _ => panic!("has not been implemented yet"),
        }
    }
}

pub struct TailDropping {
    wind: WindState,
    dropping: DroppingState,
    tail: TailState,
}

impl TailDropping {
    pub fn new(cfg: WeatherInfomation) -> Self {
        Self {
            wind: WindState::new(cfg.wind.without_random()),
            tail: TailState::new(cfg.wind.into()),
            dropping: DroppingState {
                threshold: cfg.level.unwrap_or(DEF_TAIL_LEVEL),
                mode: cfg.mode,
            },
        }
    }
}

impl WeatherImpl for TailDropping {}

impl EachFrameImpl for TailDropping {
    fn on_frame(&mut self, rb: &mut RenderBuffer, seed: u64, frame: u64) -> ShouldRender {
        self.wind
            .on_frame(rb, seed, frame)
            .or(self.dropping.on_frame(rb, seed, frame))
            .or(self.tail.on_frame(rb, seed, frame))
    }
}

impl AsWeatherWidget for TailDropping {
    type Weather = GeneralWeatherWidget;

    fn as_weather_widget(&self) -> Self::Weather {
        use Mode::*;
        match self.dropping.mode {
            Meteor => GeneralWeatherWidget::Meteor(self.tail.mode),
            _ => panic!("has not been implemented yet"),
        }
    }
}
