use std::path::PathBuf;

use ratatui::style::Color;

#[derive(clap::Parser, Debug)]
pub struct Cli {
    #[arg(short, long)]
    pub tps: Option<u8>,
    #[arg(short, long)]
    pub fps: Option<u8>,
    /// format date
    #[arg(short = 'D', long)]
    pub date: Option<String>,
    #[arg(short, long)]
    pub utc: Option<bool>,
    #[arg(short = 'd', long)]
    pub hide_date: Option<bool>,
    #[arg(short, long)]
    pub show_seconds: Option<bool>,
    #[arg(short, long)]
    pub center: Option<bool>,
    #[arg(short = 'T', long)]
    pub hour12: Option<bool>,
    #[arg(short = 'C', long)]
    pub color: Option<Color>,
    #[arg(long, value_parser = clap::value_parser!(PathBuf))]
    pub config: Option<PathBuf>,
    #[arg(long, conflicts_with = "config")]
    pub no_config: bool,
    #[arg(long, value_parser = clap::value_parser!(PathBuf))]
    pub generate_config: Option<Option<PathBuf>>,
}
