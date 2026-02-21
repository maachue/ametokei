use std::io::Stdout;

use color_eyre::eyre::Result;
use crossterm::{
    cursor,
    event::DisableMouseCapture,
    execute,
    terminal::{
        self, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
    },
};
use ratatui::{Terminal, prelude::CrosstermBackend};

use crate::{
    config::MinimalConfig,
    font::Font,
    state::{ShouldRender, State},
};

pub struct App {
    config: MinimalConfig,
    state: State,

    terminal: Terminal<CrosstermBackend<Stdout>>,
    // tui: Tui,
    quit: bool,
    should_render: ShouldRender,
}
impl App {
    pub fn new(config: MinimalConfig, font: Font) -> Result<Self> {
        enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        execute!(stdout, EnterAlternateScreen)?;

        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        let state = State::new(terminal.size()?.into(), &config, font)?;

        Ok(Self {
            state,
            config,
            terminal,
            quit: false,
            should_render: ShouldRender::Render,
        })
    }
}
impl Drop for App {
    fn drop(&mut self) {
        if crossterm::terminal::is_raw_mode_enabled().unwrap() {
            let _ = disable_raw_mode();
            let _ = execute!(
                self.terminal.backend_mut(),
                LeaveAlternateScreen,
                DisableMouseCapture,
                cursor::Show
            );
        }
    }
}
