//! msg based on btop++!
//! Thanks to btop++ team so much.

use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Widget,
};

pub struct NotEnoughWidget {
    pub needed_w: u16,
    pub needed_h: u16,
}
impl Widget for NotEnoughWidget {
    fn render(self, _: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        buf.set_string(
            (buf.area.width / 2) - 11,
            (buf.area.height / 2) - 2,
            "Terminal size too small:",
            Color::Reset,
        );

        let fg_width = if buf.area.width < self.needed_w {
            Color::Red
        } else {
            Color::Green
        };

        let fg_height = if buf.area.height < self.needed_h {
            Color::Red
        } else {
            Color::Green
        };

        let line = Line::from(vec![
            Span::raw("Width = "),
            Span::styled(format!("{}", self.needed_w), Style::default().fg(fg_width)),
            Span::raw(" Height = "),
            Span::styled(format!("{}", self.needed_h), Style::default().fg(fg_height)),
        ]);

        buf.set_line(
            (buf.area.width / 2) - 10,
            (buf.area.height / 2) - 1,
            &line,
            buf.area.width,
        );
        buf.set_string(
            (buf.area.width / 2) - 12,
            (buf.area.height / 2) + 1,
            "Needed for current config:",
            Modifier::BOLD,
        );
        buf.set_string(
            (buf.area.width / 2) - 10,
            (buf.area.height / 2) + 2,
            format!("Width = {} Height = {}", self.needed_w, self.needed_h),
            Modifier::BOLD,
        );
    }
}
