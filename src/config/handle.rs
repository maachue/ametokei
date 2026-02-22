use color_eyre::eyre::{OptionExt, Result};

use crate::font::Font;

use super::{Config as UserConfig, MinimalConfig};

impl From<UserConfig> for MinimalConfig {
    fn from(user: UserConfig) -> Self {
        Self {
            center: user.general.center,
            color: user.general.color,
            format_date: user.general.format,
            show_date: !user.general.hide_date,
            fps: user.performance.fps,
            tps: user.performance.tps,
            utc: user.general.utc,
            hour12: user.general.format_12h,
            sec: user.general.show_seconds,
            spacing: (
                user.fontconfig.spacing_width_between_digits,
                user.fontconfig.spacing_between_timer_and_date,
            ),
            mer: user.general.merdiem,
        }
    }
}

impl UserConfig {
    pub fn convert(mut self /* move */) -> Result<(MinimalConfig, Font)> {
        let font = match self.general.font.as_str() {
            "digital" => Font::digital(),
            _ => {
                let font_name = self.general.font.clone();

                let mut fonts = self
                    .fonts
                    .take()
                    .ok_or_eyre("User fonts are not defined.")?;
                let user_font = fonts
                    .remove(&font_name)
                    .ok_or_eyre(format!("Font `{}` not found", font_name))?;

                user_font.try_into()?
            }
        };

        Ok((self.into(), font))
    }
}
