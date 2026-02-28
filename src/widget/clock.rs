use itertools::Itertools;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Color,
    widgets::Widget,
};
use unicode_width::UnicodeWidthStr;

use crate::{
    font::Font,
    state::clock::{Clock, ClockState},
};

const NOT_ENOUGH_SIZE: &str = "not enough size, pls resize to continue";
const SIZE: u16 = 39;

pub struct ClockWidget<'a> {
    pub state: &'a ClockState,
    pub time: &'a Clock,
    pub font: &'a Font,
    pub color: Color,
}
impl<'a> ClockWidget<'a> {
    pub fn render_colon(&self, area: Rect, buf: &mut Buffer) {
        let left = area.left();
        let top = area.top();

        self.font.digits[10]
            .iter()
            .chunks(self.font.colon_width as usize)
            .into_iter()
            .enumerate()
            .for_each(|(y, chunk)| {
                chunk.into_iter().enumerate().for_each(|(x, c)| {
                    let char = if *c == 0 {
                        ' '
                    } else {
                        self.font.symbols[*c as usize - 1]
                    };

                    // maybe panic (cell_mut return Option for less panic but i dont like it)
                    buf[(left + x as u16, top + y as u16)]
                        .set_char(char)
                        .set_fg(self.color);
                });
            });
    }

    fn render_number(&self, d: u8, area: Rect, buf: &mut Buffer) {
        let left = area.left();
        let top = area.top();

        self.font.digits[d as usize]
            .iter()
            .chunks(self.font.width as usize)
            .into_iter()
            .enumerate()
            .for_each(|(y, chunk)| {
                chunk.into_iter().enumerate().for_each(|(x, c)| {
                    let char = if *c == 0 {
                        ' '
                    } else {
                        self.font.symbols[*c as usize - 1]
                    };

                    buf[(left + x as u16, top + y as u16)]
                        .set_char(char)
                        .set_fg(self.color);
                });
            });
    }

    pub fn render_decimal(&self, d: u8, area: Rect, buf: &mut Buffer) {
        let layout = Layout::new(
            Direction::Horizontal,
            Constraint::from_lengths([self.font.width, self.state.spacing.1, self.font.width]),
        )
        .split(area);

        self.render_number(d / 10, layout[0], buf);
        self.render_number(d % 10, layout[2], buf);
    }
}
impl<'a> Widget for ClockWidget<'a> {
    fn render(self, _: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let layout = Layout::new(
            Direction::Vertical,
            [
                Constraint::Length(self.font.height),
                Constraint::Length(self.state.spacing.1),
                Constraint::Min(0),
            ],
        )
        .split(self.state.area);

        if self.state.show_seconds {
            let [hour_digit, _, colon1, _, min_digit, _, colon2, _, sec_digit] = Layout::new(
                Direction::Horizontal,
                Constraint::from_lengths([
                    self.font.width * 2 + self.state.spacing.0,
                    self.state.spacing.0,
                    self.font.colon_width,
                    self.state.spacing.0,
                    self.font.width * 2 + self.state.spacing.0,
                    self.state.spacing.0,
                    self.font.colon_width,
                    self.state.spacing.0,
                    self.font.width * 2 + self.state.spacing.0,
                ]),
            )
            .areas(layout[0]);

            self.render_decimal(self.time.hours, hour_digit, buf);
            self.render_decimal(self.time.minutes, min_digit, buf);
            self.render_decimal(self.time.seconds, sec_digit, buf);

            if self.state.colon_show {
                self.render_colon(colon1, buf);
                self.render_colon(colon2, buf);
            }
        } else {
            let [hour_digit, _, colon, _, min_digit] = Layout::new(
                Direction::Horizontal,
                Constraint::from_lengths([
                    self.font.width * 2 + self.state.spacing.0,
                    self.state.spacing.0,
                    self.font.colon_width,
                    self.state.spacing.0,
                    self.font.width * 2 + self.state.spacing.0,
                ]),
            )
            .areas(layout[0]);

            self.render_decimal(self.time.hours, hour_digit, buf);
            self.render_decimal(self.time.minutes, min_digit, buf);

            if self.state.colon_show {
                self.render_colon(colon, buf);
            }
        }

        if let Some(date_str) = &self.time.date {
            let date = layout[2];
            let x = date.width.saturating_sub(date_str.width() as u16) / 2;

            buf.set_string(date.left() + x, date.top(), date_str, self.color);
        }
    }
}
