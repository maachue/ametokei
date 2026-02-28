use ratatui::{Frame, style::Color};

use crate::{state::{EnoughSize, State}, widget::{clock::ClockWidget, notenough::NotEnoughWidget}};

pub fn ui(f: &mut Frame, state: &mut State, color: Color) {
    let area = f.area();


    if state.enough_size.is_enough() {
        f.render_widget(
            ClockWidget {
                state: &state.clock_state,
                time: &state.clock,
                color,
                font: &state.font,
            },
            area,
        );
    } else {
        let EnoughSize::Not(needed_w, needed_h) = state.enough_size else {
            unreachable!()
        };

        f.render_widget(NotEnoughWidget { needed_h, needed_w }, area);
    }
}
