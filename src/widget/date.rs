use ratatui::{style::Color, widgets::Widget};

use crate::state::date::DateState;

pub struct DateWidget<'a> {
    pub state: &'a DateState,
    pub date: &'a str,
    pub color: Color,
}

impl<'a> Widget for DateWidget<'a> {
    fn render(self, _: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        buf.set_string(
            self.state.padding,
            self.state.area.top(),
            &self.date,
            self.color,
        );
    }
}
