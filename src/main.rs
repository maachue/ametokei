use std::path::{Path, PathBuf};

use clap::Parser;
use color_eyre::eyre::{OptionExt, Result};

use crate::{
    app::App,
    config::{Config, MinimalConfig},
    font::Font,
};

mod app;
mod cli;
mod config;
mod font;
mod state;
mod tui;
mod ui;
mod widget;

#[inline]
fn get_config() -> Option<PathBuf> {
    directories::ProjectDirs::from("com", "maachue", "jikan")
        .map(|f| f.config_dir().join("config.toml"))
}

fn resolve_config_path(cmd: &cli::Cli) -> Result<Option<PathBuf>> {
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

fn config_load(cmd: &cli::Cli) -> Result<(MinimalConfig, Font)> {
    if cmd.no_config {
        let mut cfg = MinimalConfig::default();
        cfg.cmd(cmd);
        return Ok((cfg, Font::digital()));
    }

    let config_path = resolve_config_path(cmd)?;

    let (mut cfg, font) = if let Some(path) = config_path {
        let user = Config::from_path(&path)?;
        let (_, cfg, font) = user.convert()?;
        (cfg, font)
    } else {
        (MinimalConfig::default(), Font::digital())
    };

    cfg.cmd(cmd);

    Ok((cfg, font))
}

fn config_gen(maybe_default: Option<&Path>) -> Result<()> {
    let default_config = Config::default();

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

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let cmd = cli::Cli::parse();

    if let Some(maybe_default /* None -> Default */) = &cmd.generate_config {
        config_gen(maybe_default.as_deref())?;
        return Ok(()) // do not run clock
    }

    let (config, font) = config_load(&cmd)?;

    println!("{:?}", cmd);
    println!("{:?}", config);

    let mut app = App::new(config, font)?;
    app.run().await?;

    Ok(())
}
