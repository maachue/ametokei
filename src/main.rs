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
    color_eyre::install()?;

    let cmd = cli::Cli::parse();
    let mut config = MinimalConfig::default();
    config.cmd(&cmd);
    let font = Font::digital();

    let mut app = App::new(config, font)?;
    app.run().await?;

    Ok(())
}
