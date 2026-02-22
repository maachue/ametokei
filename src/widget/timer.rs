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
    pub timer: &'a crate::state::timer::Timer,
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
        let [
            num1,
            _, /* padding width */
            num2,
            _, /* padding width */
        ] = Layout::new(
            ratatui::layout::Direction::Horizontal,
            Constraint::from_lengths([
                self.font.width,
                self.state.spacing.0,
                self.font.width,
                self.state.spacing.0,
            ]),
        )
        .areas(area);

        Self::render_number(&self.font.digits[d as usize / 10], num1, self.color, buf);
        Self::render_number(&self.font.digits[d as usize % 10], num2, self.color, buf);
    }
}

impl<'a> Widget for TimerWidget<'a> {
    fn render(self, _: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let [area, _ /* spacing height */] = Layout::new(
            ratatui::layout::Direction::Vertical,
            Constraint::from_lengths([self.font.height, self.state.spacing.1]),
        )
        .areas(self.state.area);

        if !self.state.show_sec {
            let [hours, colon, _ /* spacing width */, minutes] = Layout::new(
                ratatui::layout::Direction::Horizontal,
                Constraint::from_lengths([
                    (self.font.width + self.state.spacing.0) * 2,
                    self.font.colon.width,
                    self.state.spacing.0,
                    (self.font.width + self.state.spacing.0) * 2,
                ]),
            )
            .areas(area);

            self.render_decimal(self.timer.hours, hours, buf);
            Self::render_colon(&self.font.colon, colon, self.color, buf);
            self.render_decimal(self.timer.minutes, minutes, buf);
        } else {
            let [
                hours,
                colon,
                _, /* spacing width */
                minutes,
                colon1,
                _, /* spacing width */
                seconds,
            ] = Layout::new(
                ratatui::layout::Direction::Horizontal,
                Constraint::from_lengths([
                    (self.font.width + self.state.spacing.0) * 2,
                    self.font.colon.width,
                    self.state.spacing.0,
                    (self.font.width + self.state.spacing.0) * 2,
                    self.font.colon.width,
                    self.state.spacing.0,
                    (self.font.width + self.state.spacing.0) * 2,
                ]),
            )
            .areas(area);

            self.render_decimal(self.timer.hours, hours, buf);
            Self::render_colon(&self.font.colon, colon, self.color, buf);
            self.render_decimal(self.timer.minutes, minutes, buf);
            Self::render_colon(&self.font.colon, colon1, self.color, buf);
            self.render_decimal(self.timer.seconds, seconds, buf);
        }
    }
}
