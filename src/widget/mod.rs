use std::borrow::Cow;

use color_eyre::eyre::Result;
use ratatui::layout::{Layout, Rect};
use unicode_width::UnicodeWidthStr;

use crate::{font::Font, state::timer::TimerState};

pub mod date;
pub mod timer;

#[derive(thiserror::Error, Debug)]
pub enum LayoutErr {
    #[error(
        "The size of terminal is too small for {context} (expected: > ({ex_w};{ex_h}); current ({term_w};{term_h})"
    )]
    SizeTerm {
        context: Cow<'static, str>,
        term_w: u16,
        term_h: u16,
        ex_w: u16,
        ex_h: u16,
    },
}

// idk, just a random name
pub struct Layouted {
    pub timer: Rect,
    pub date: Option<Rect>,
}
impl Layouted {
    pub fn get(
        size: Rect,
        spacing: (u16, u16),
        center: bool,
        show_sec: bool,
        font: &Font,
        date: Option<&str>,
    ) -> Result<Self, LayoutErr> {
        if font.width > size.width || font.height > size.height {
            return Err(LayoutErr::SizeTerm {
                context: Cow::Borrowed("font"),
                ex_w: font.width,
                ex_h: font.height,
                term_w: size.width,
                term_h: size.height,
            });
        };

        let (timer_w, timer_h) = TimerState::get_size(spacing, show_sec, font);
        let date_w = if let Some(date) = date {
            date.width() as u16
        } else {
            0
        };

        // use date width or timer width (date width maybe > timer width)
        let main_w = if timer_w >= date_w { timer_w } else { date_w };
        let main_h = if date.is_some() { timer_h + 1 } else { timer_h };

        let padding_w = if center {
            size.width.saturating_sub(main_w) / 2
        } else {
            0
        };
        let padding_h = if center {
            size.height.saturating_sub(main_h) / 2
        } else {
            0
        };

        let main_area = Rect {
            width: main_w,
            height: main_h,
            x: padding_w,
            y: padding_h,
        };

        if date.is_some() {
            let [timer, date] = Layout::new(
                ratatui::layout::Direction::Vertical,
                ratatui::layout::Constraint::from_lengths([timer_h, 1]),
            )
            .areas(main_area);

            Ok({
                Self {
                    timer,
                    date: Some(date),
                }
            })
        } else {
            Ok(Self {
                timer: main_area,
                date: None,
            })
        }
    }
}
