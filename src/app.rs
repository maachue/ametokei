use std::io::Stdout;

use color_eyre::eyre::Result;
use crossterm::{
    cursor,
    event::{DisableMouseCapture, KeyCode, KeyEvent},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, prelude::CrosstermBackend};

use crate::{
    config::MinimalConfig,
    font::Font,
    state::{ShouldRender, State},
    tui::Tui,
};

pub struct App {
    config: MinimalConfig,
    state: State,
    terminal: Terminal<CrosstermBackend<Stdout>>,
    tui: Tui,
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

        let tui = crate::tui::Tui::new(config.fps as f64, config.tps as f64);

        let state = State::new(terminal.size()?.into(), &config, font)?;

        Ok(Self {
            config,
            state,
            terminal,
            quit: false,
            should_render: ShouldRender::Render,
            tui,
        })
    }
    pub async fn run(&mut self) -> Result<()> {
        use crate::tui::Event;
        self.tui.run();

        loop {
            if let Some(evt) = self.tui.next().await {
                match evt {
                    Event::Init => {}
                    /* Event::Quit | */ Event::Error => self.quit = true,
                    Event::Render => self.on_render()?,
                    Event::Tick => {}
                    Event::Timer => self.on_timer(),
                    Event::Key(key) => self.on_key(key),
                    Event::Resize(w, h) => self.on_resize(w, h)?,
                }

                if self.quit {
                    break;
                }
            }
        }

        Ok(())
    }

    fn on_render(&mut self) -> Result<()> {
        if self.should_render.is_render() {
            self.should_render = ShouldRender::Skip;
            self.terminal
                .draw(|f| crate::ui::ui(f, &mut self.state, self.config.color))?;
        }

        Ok(())
    }

    fn on_timer(&mut self) {
        self.state.tick_timer(&self.config);
        self.should_render = ShouldRender::Render;
    }

    fn on_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => self.quit = true,
            _ => {}
        }
    }

    fn on_resize(&mut self, w: u16, h: u16) -> Result<()> {
        self.should_render = ShouldRender::Render;

        self.state.on_resize(w, h, &self.config)
    }
}
impl Drop for App {
    fn drop(&mut self) {
        if crossterm::terminal::is_raw_mode_enabled().unwrap() {
            let _ = execute!(
                self.terminal.backend_mut(),
                LeaveAlternateScreen,
                DisableMouseCapture,
                cursor::Show
            );
            let _ = disable_raw_mode();
        }
    }
}
