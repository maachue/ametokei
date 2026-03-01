#![allow(clippy::manual_is_multiple_of)]

use std::path::{Path, PathBuf};

use clap::Parser;
use color_eyre::Result;

use crate::{
    app::App,
    cli::Cli,
    config::{RuntimeConfig, user::UserConfig},
    font::Font,
};

mod app;
mod cli;
mod config;
mod font;
mod state;
mod tui;
mod ui;
mod weather;
mod widget;

#[cfg(feature = "tracing")]
#[inline(always)]
fn init_tracing() {
    use tracing_appender::rolling;
    use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

    let file_appender = rolling::never(".", "tui.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::registry()
        .with(
            fmt::layer()
                .with_writer(non_blocking)
                .with_target(true)
                .with_line_number(true)
                .with_thread_ids(true),
        )
        .init();
}

#[inline]
fn get_config() -> Option<PathBuf> {
    directories::ProjectDirs::from("com", "maachue", "ametokei")
        .map(|p| p.config_dir().join("config.toml"))
}

fn resolve_path(cmd: &Cli) -> Result<Option<PathBuf>> {
    if let Some(p) = &cmd.config {
        if p.is_dir() {
            return Err(color_eyre::eyre::eyre!(
                "Config path must be a file: {}",
                p.display()
            ));
        }
        if !p.exists() {
            return Err(color_eyre::eyre::eyre!(
                "Config file not found at: {}",
                p.display()
            ));
        }
        return Ok(Some(p.clone()));
    }

    if let Some(p) = get_config()
        && p.exists()
    {
        if p.is_dir() {
            return Err(color_eyre::eyre::eyre!(
                "Bro, why do bro touch my default config file into a directory?"
            ));
        }
        return Ok(Some(p));
    }

    Ok(None)
}

fn config_load(cmd: Cli) -> Result<(RuntimeConfig, Font)> {
    if cmd.no_config {
        let mut cfg = RuntimeConfig::default();

        let font = if let Some(font_name) = &cmd.font {
            Font::get(font_name, None)?
        } else {
            Font::tenki()
        };

        cfg.cli_override(cmd);

        return Ok((cfg, font));
    }

    let config_path = resolve_path(&cmd)?;

    let (mut cfg, font) = if let Some(path) = config_path {
        let mut user = UserConfig::from_path(&path)?;

        if let Some(custom_font) = &cmd.font {
            user.general.font = custom_font.clone()
        };

        let font = Font::get(&user.general.font, user.font.as_mut())?;

        (user.into(), font)
    } else {
        let font = if let Some(font_name) = &cmd.font {
            Font::get(font_name, None)?
        } else {
            Font::tenki()
        };

        (RuntimeConfig::default(), font)
    };

    cfg.cli_override(cmd);

    Ok((cfg, font))
}

fn config_gen(maybe_default: Option<&Path>) -> Result<()> {
    use color_eyre::eyre::OptionExt;

    let default_config = UserConfig::default();

    let config = if let Some(config) = maybe_default {
        if config.exists() {
            return Err(color_eyre::eyre::eyre!(
                "Cannot create config file the path already exists: {}",
                config.display()
            ));
        }
        if config.is_dir() {
            return Err(color_eyre::eyre::eyre!(
                "Cannot create config file because the path is a directory: {}",
                config.display()
            ));
        }
        config.to_path_buf()
    } else {
        get_config().ok_or_eyre("Failed to determine default config directory.")?
    };

    if let Some(parent) = config.parent() {
        std::fs::create_dir_all(parent)?;
    }

    std::fs::write(&config, toml::to_string_pretty(&default_config)?)?;
    println!("Complete generate config at: {}", config.display());
    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    color_eyre::install()?;

    #[cfg(feature = "tracing")]
    init_tracing();

    let cli = Cli::parse();

    if let Some(maybe_default) = &cli.generate_config {
        config_gen(maybe_default.as_deref())?;
        return Ok(());
    }

    let (rt_config, font) = config_load(cli)?;

    let weather = weather::Weather::from(rt_config.weather_info); // copy

    let mut app = App::new(rt_config, weather, font)?;
    app.run().await?;

    Ok(())
}
