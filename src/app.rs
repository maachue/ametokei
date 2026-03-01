use color_eyre::eyre::Result;
use crossterm::{
    event::{DisableMouseCapture, KeyEvent},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, prelude::CrosstermBackend};

use crate::{
    config::RuntimeConfig,
    font::Font,
    state::{EachFrameImpl, ShouldRender, State},
    tui::Tui,
    weather::Weather,
    widget::AsWeatherWidget,
};

pub struct App<T> {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    tui: Tui,
    state: State<T>,
    quit: bool,
    should_render: ShouldRender,
    config: RuntimeConfig,
}
impl<T> App<T>
where
    T: EachFrameImpl + AsWeatherWidget,
{
    pub fn new(config: RuntimeConfig, weather: T, font: Font) -> Result<Self> {
        enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        execute!(stdout, EnterAlternateScreen)?;

        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        let state = State::new(terminal.size()?.into(), weather, &config, font);

        Ok(Self {
            terminal,
            tui: Tui::new(config.fps as f64, config.tps as f64),
            state,
            quit: false,
            should_render: ShouldRender::Render,
            config,
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        use crate::tui::Event;
        self.tui.run();

        loop {
            if let Some(evt) = self.tui.next().await {
                match evt {
                    Event::Init => {}
                    /* Event::Quit || */ Event::Error => self.quit = true,
                    Event::Render => self.on_render()?,
                    Event::Tick => self.on_tick(),
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
        if self.config.fps == self.config.tps {
            self.on_tick()
        }

        if self.should_render.is_render() {
            self.should_render = ShouldRender::Skip;
            self.terminal
                .draw(|f| crate::ui::ui(f, &mut self.state, self.config.color))?;
        }

        Ok(())
    }

    fn on_tick(&mut self) {
        self.should_render = self.should_render.or(self.state.tick());
        // self.frame_in_second = self.frame_in_second.saturating_add(1);
    }

    fn on_timer(&mut self) {
        self.state.tick_timer(&self.config);
        self.should_render = ShouldRender::Render;
    }

    fn on_key(&mut self, key: KeyEvent) {
        use crossterm::event::KeyCode;

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
impl<T> Drop for App<T> {
    fn drop(&mut self) {
        if crossterm::terminal::is_raw_mode_enabled().unwrap() {
            let _ = execute!(
                self.terminal.backend_mut(),
                LeaveAlternateScreen,
                DisableMouseCapture,
                crossterm::cursor::Show
            );
            let _ = disable_raw_mode();
        }
    }
}
