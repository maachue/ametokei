use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Color,
    widgets::Widget,
};

use crate::{
    font::{Colon, Font},
    state::timer::TimerState,
};

pub struct TimerWidget<'a> {
    pub timer: crate::state::timer::Timer,
    pub color: Color,
    pub state: &'a TimerState,
    pub font: &'a Font,
}

impl<'a> TimerWidget<'a> {
    pub fn render_colon(colon: &Colon, area: Rect, color: Color, buf: &mut Buffer) {
        let x = area.left();
        let top = area.top();

        colon.lines.iter().enumerate().for_each(|(y, string)| {
            buf.set_string(x, top + y as u16, string, color);
        });
    }
    fn render_number(digit: &[String], area: Rect, color: Color, buf: &mut Buffer) {
        let x = area.left();
        let top = area.top();

        digit.iter().enumerate().for_each(|(y, string)| {
            buf.set_string(x, top + y as u16, string, color);
        });
    }

    pub fn render_decimal(&self, d: u8, area: Rect, buf: &mut Buffer) {
        let layout = Layout::new(
            ratatui::layout::Direction::Horizontal,
            Constraint::from_lengths([self.font.width, self.font.width]),
        )
        .split(area);

        Self::render_number(
            &self.font.digits[d as usize / 10],
            layout[0],
            self.color,
            buf,
        );
        Self::render_number(
            &self.font.digits[d as usize % 10],
            layout[1],
            self.color,
            buf,
        );
    }
}

impl<'a> Widget for TimerWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        if self.state.show_sec {
            let [hours, colon, minutes] = Layout::new(
                ratatui::layout::Direction::Horizontal,
                Constraint::from_lengths([
                    self.font.width * 2,
                    self.font.colon.width,
                    self.font.width * 2,
                ]),
            )
            .areas(self.state.area);

            self.render_decimal(self.timer.hours, hours, buf);
            Self::render_colon(&self.font.colon, colon, self.color, buf);
            self.render_decimal(self.timer.minutes, minutes, buf);
        } else {
            let [hours, colon, minutes, colon1, seconds] = Layout::new(
                ratatui::layout::Direction::Horizontal,
                Constraint::from_lengths([
                    self.font.width * 2,
                    self.font.colon.width,
                    self.font.width * 2,
                    self.font.colon.width,
                    self.font.width * 2,
                ]),
            )
            .areas(self.state.area);

            self.render_decimal(self.timer.hours, hours, buf);
            Self::render_colon(&self.font.colon, colon, self.color, buf);
            self.render_decimal(self.timer.minutes, minutes, buf);
            Self::render_colon(&self.font.colon, colon1, self.color, buf);
            self.render_decimal(self.timer.seconds, seconds, buf);
        }
    }
}
