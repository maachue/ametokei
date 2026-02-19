use color_eyre::eyre::Result;

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

    Ok(())
}
