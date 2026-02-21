use ratatui::{style::Color, widgets::Widget};
use unicode_width::UnicodeWidthStr;

use crate::state::date::DateState;

pub struct DateWidget<'a> {
    pub state: &'a DateState,
    pub color: Color,
}

impl<'a> Widget for DateWidget<'a> {
    fn render(self, _: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        // always center
        let padding = self
            .state
            .area
            .width
            .saturating_sub(self.state.date.0.width() as u16)
            / 2;

        buf.set_string(
            padding,
            self.state.area.top(),
            &self.state.date.0,
            self.color,
        );
    }
}
