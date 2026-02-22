use ratatui::style::Color;

#[derive(clap::Parser)]
pub struct Cli {
    #[arg(short, long)]
    pub tps: Option<u8>,
    #[arg(short, long)]
    pub fps: Option<u8>,
    /// format date
    #[arg(short = 'D', long)]
    pub date: Option<String>,
    #[arg(short, long)]
    pub utc: bool,
    #[arg(short = 'd', long)]
    pub hide_date: bool,
    #[arg(short, long)]
    pub center: bool,
    #[arg(short = 'T', long)]
    pub hour12: bool,
    #[arg(short = 'C', long)]
    pub color: Option<Color>,
    // #[arg(long, value_parser = clap::value_parser!(PathBuf))]
    // pub config: Option<PathBuf>,
}
