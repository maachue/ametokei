use ratatui::{Frame, style::Color};

use crate::{
    state::{EachFrameImpl, EnoughSize, State},
    widget::{AsWeatherWidget, WeatherWidget, clock::ClockWidget, notenough::NotEnoughWidget},
};

pub fn ui<T: EachFrameImpl + AsWeatherWidget>(f: &mut Frame, state: &mut State<T>, color: Color) {
    let _ = crossterm::execute!(std::io::stdout(), crossterm::terminal::BeginSynchronizedUpdate);

    let area = f.area();

    if state.enough_size.is_enough() {
        f.render_stateful_widget(
            WeatherWidget::new(state.weather.as_weather_widget()),
            area,
            &mut state.rb,
        );

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

    let _ = crossterm::execute!(std::io::stdout(), crossterm::terminal::EndSynchronizedUpdate);
}
