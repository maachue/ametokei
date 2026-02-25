use color_eyre::Result;

use crate::app::App;

mod app;
mod config;
mod font;
mod state;
mod tui;
mod ui;
mod widget;

use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};
use tracing_appender::rolling;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    // ---------- tracing init ----------
    let file_appender = rolling::never(".", "tui.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::registry()
        .with(
            fmt::layer()
                .with_writer(non_blocking)
                .with_target(true)
                .with_line_number(true)
                .with_thread_ids(true)
        )
        .init();
    // ----------------------------------

    let font = crate::font::Font::tenki();
    let mut rt_config = crate::config::RuntimeConfig::default();

    // test config edit here
    rt_config.blink_colon = true;
    // rt_config.center = true;
    // rt_config.show_seconds = true;
    rt_config.hour12h = true;
    rt_config.utc = true;
    // rt_config.timer_mode = Some(crate::state::clock::TimerRenderMode::Dvd(crate::state::Direction::default()));

    let mut app = App::new(rt_config, font)?;
    app.run().await?;

    Ok(())
}
