use ratatui::{style::Color, widgets::Widget};

pub struct DateWidget<'a> {
    pub date: &'a str,
    pub color: Color,
}

impl<'a> Widget for DateWidget<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let left = area.left();
        let top = area.top();

        buf.set_string(left, top, self.date, self.color);
    }
}
