//! msg based on btop++!
//! Thanks to btop++ team so much.

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
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
        let area = buf.area;

        let fg_width = if area.width < self.needed_w {
            Color::Red
        } else {
            Color::Green
        };
        let fg_height = if area.height < self.needed_h {
            Color::Red
        } else {
            Color::Green
        };

        let text = vec![
            Line::from("Terminal size too small:").alignment(Alignment::Center),
            Line::from(vec![
                Span::raw("Width = "),
                Span::styled(format!("{}", area.width), Style::default().fg(fg_width)),
                Span::raw(" Height = "),
                Span::styled(format!("{}", area.height), Style::default().fg(fg_height)),
            ])
            .alignment(Alignment::Center),
            Line::from(""),
            Line::from("Needed for current config:")
                .style(Modifier::BOLD)
                .alignment(Alignment::Center),
            Line::from(format!(
                "Width = {} Height = {}",
                self.needed_w, self.needed_h
            ))
            .style(Modifier::BOLD)
            .alignment(Alignment::Center),
        ];

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(5),
                Constraint::Fill(1),
            ])
            .split(area);

        Paragraph::new(text).render(chunks[1], buf);
    }
}
