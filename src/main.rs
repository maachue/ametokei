use std::panic;

use clap::Parser;
use color_eyre::eyre::Result;

use crate::{app::App, config::MinimalConfig, font::Font};

mod app;
mod cli;
mod config;
mod font;
mod state;
mod tui;
mod ui;
mod widget;

#[tokio::main]
async fn main() -> Result<()> {
    panic::set_hook(Box::new(|info| {
        let _ = crossterm::terminal::disable_raw_mode();
        let _ = crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen);
        eprintln!("\n\n================ PANIC ================");
        eprintln!("{info}");
        eprintln!("=======================================\n\n");
    }));
    color_eyre::install()?;

    let cmd = cli::Cli::parse();
    let mut config = MinimalConfig::default();
    config.cmd(&cmd);
    let font = Font::digital();

    let mut app = App::new(config, font)?;
    app.run().await?;

    Ok(())
}
