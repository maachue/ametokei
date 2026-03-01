use super::{EachFrameImpl, ShouldRender, buffer::RenderBuffer};

#[derive(
    Clone, Copy, Default, Eq, PartialEq, serde::Serialize, serde::Deserialize, clap::ValueEnum,
)]
#[serde(rename_all = "snake_case")]
pub enum WindMode {
    #[default]
    Random,
    Disable,
    OnlyRight,
    OnlyLeft,
}

impl WindMode {
    pub fn without_random(self) -> Self {
        match self {
            WindMode::Random => Self::Disable,
            other => other,
        }
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub enum WindDirection {
    Left,
    Right,
    #[default]
    None,
}

type FrameCount = usize;

#[derive(Clone, Copy, Default)]
pub struct WindState {
    pub mode: WindMode,
    pub direction: WindDirection,
    frame: FrameCount,
}

impl WindState {
    pub fn new(mode: WindMode) -> Self {
        Self {
            mode,
            direction: Self::direction_from_mode(mode),
            ..Default::default()
        }
    }

    pub fn direction_from_mode(mode: WindMode) -> WindDirection {
        match mode {
            WindMode::OnlyRight => WindDirection::Right,
            WindMode::OnlyLeft => WindDirection::Left,
            _ => WindDirection::None,
        }
    }
}

impl EachFrameImpl for WindState {
    fn on_frame(&mut self, rb: &mut RenderBuffer, seed: u64, _: u64) -> ShouldRender {
        self.direction = match self.mode {
            WindMode::Disable => WindDirection::None,
            WindMode::OnlyLeft => WindDirection::Left,
            WindMode::OnlyRight => WindDirection::Right,
            _ => self.direction,
        };

        if self.mode == WindMode::Disable {
            return ShouldRender::Skip;
        }

        if self.frame == 0 || self.direction == WindDirection::None {
            self.frame = 255;
            self.direction = if seed % 2024 == 0 {
                WindDirection::Left
            } else if seed % 123 == 0 {
                WindDirection::Right
            } else {
                WindDirection::None
            }
        }

        if self.direction == WindDirection::None {
            return ShouldRender::Skip;
        }

        self.frame = self.frame.saturating_sub(1);

        if self.direction == WindDirection::Left {
            rb.buf.reverse();
        }

        for i in 1..rb.buf.len() {
            rb.buf.swap(0, i);
        }

        if self.direction == WindDirection::Left {
            rb.buf.reverse();
        }

        ShouldRender::Render
    }
}
