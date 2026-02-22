use ratatui::{Frame, style::Color};

use crate::{
    state::State,
    widget::{date::DateWidget, timer::TimerWidget},
};

pub fn ui(f: &mut Frame, state: &mut State, color: Color) {
    let area = f.area();

    f.render_widget(
        TimerWidget {
            timer: &state.timer,
            color,
            font: &state.font,
            state: &state.timer_state,
        },
        area,
    );

    if let Some(date_state) = &state.date_state
        && let Some(date) = &state.date
    {
        f.render_widget(
            DateWidget {
                state: date_state,
                date,
                color,
            },
            area,
        );
    }
}
