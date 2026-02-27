use std::path::PathBuf;

use ratatui::style::Color;

use crate::state::clock::TimerMode;

#[derive(clap::Parser, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// frame per seconds [default: 60]
    #[arg(short, long)]
    pub fps: Option<u8>,

    /// tick per seconds [default: 60]
    #[arg(short, long)]
    pub tps: Option<u8>,

    /// set the hour in 12h format
    #[arg(long)]
    pub hour12: Option<bool>,

    /// center of the terminal
    #[arg(short, long)]
    pub center: Option<bool>,

    /// use UTC time
    #[arg(long)]
    pub utc: Option<bool>,

    /// show seconds
    #[arg(long)]
    pub show_seconds: Option<bool>,

    /// blink colon of timer
    #[arg(long)]
    pub blink_colon: Option<bool>,

    /// set the date format [default: "%Y-%m-%d"]
    #[arg(long)]
    pub format_date: Option<String>,

    /// hide date
    #[arg(long)]
    pub hide_date: bool,

    /// color of the clock
    #[arg(long)]
    pub timer_color: Option<Color>,

    #[arg(long)]
    pub timer_mode: Option<TimerMode>,

    #[arg(long)]
    pub font: Option<String>,

    // ------------ CLI ------------
    /// custom config path
    #[arg(long, value_parser = clap::value_parser!(PathBuf))]
    pub config: Option<PathBuf>,

    /// no-config mode
    #[arg(long, conflicts_with = "config")]
    pub no_config: bool,

    /// create config
    #[arg(long, value_parser = clap::value_parser!(PathBuf))]
    pub generate_config: Option<Option<PathBuf>>,
}
