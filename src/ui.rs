use ratatui::{Frame, style::Color};

use crate::{state::State, widget::clock::ClockWidget};

pub fn ui(f: &mut Frame, state: &mut State, color: Color) {
    let area = f.area();

    f.render_widget(
        ClockWidget {
            state: &state.clock_state,
            time: &state.clock,
            color,
            font: &state.font,
        },
        area,
    );
}
