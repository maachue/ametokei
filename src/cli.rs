use std::path::PathBuf;

use ratatui::style::Color;

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// tick per seconds [default: 60]
    #[arg(short, long)]
    pub tps: Option<u8>,

    /// frame per seconds [default: 60]
    #[arg(short, long)]
    pub fps: Option<u8>,

    /// set the date format [default: "%Y-%m-%d"]
    #[arg(long)]
    pub date: Option<String>,

    /// use UTC time
    #[arg(short, long)]
    pub utc: Option<bool>,

    /// hide date
    #[arg(short = 'D', long)]
    pub hide_date: Option<bool>,

    /// show seconds
    #[arg(short, long)]
    pub show_seconds: Option<bool>,

    /// center of the terminal
    #[arg(short, long)]
    pub center: Option<bool>,

    /// set the hour in 12h format
    #[arg(long)]
    pub hour12: Option<bool>,

    /// color of the timer & date
    #[arg(long)]
    pub timer_color: Option<Color>,

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
