use color_eyre::eyre::{OptionExt, Result};

use crate::{
    config::MeridiemConfig,
    font::{Colon, Font},
};

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
        }
    }
}

impl UserConfig {
    pub fn convert(mut self /* move */) -> Result<(MeridiemConfig, MinimalConfig, Font)> {
        let font = match self.general.font.as_str() {
            "digital" => Font {
                width: 6,
                height: 5,
                digits: [
                    vec![
                        "██████".to_string(),
                        "██  ██".to_string(),
                        "██  ██".to_string(),
                        "██  ██".to_string(),
                        "██████".to_string(),
                    ],
                    vec![
                        "    ██".to_string(),
                        "    ██".to_string(),
                        "    ██".to_string(),
                        "    ██".to_string(),
                        "    ██".to_string(),
                    ],
                    vec![
                        "██████".to_string(),
                        "    ██".to_string(),
                        "██████".to_string(),
                        "██    ".to_string(),
                        "██████".to_string(),
                    ],
                    vec![
                        "██████".to_string(),
                        "    ██".to_string(),
                        "██████".to_string(),
                        "    ██".to_string(),
                        "██████".to_string(),
                    ],
                    vec![
                        "██  ██".to_string(),
                        "██  ██".to_string(),
                        "██████".to_string(),
                        "    ██".to_string(),
                        "    ██".to_string(),
                    ],
                    vec![
                        "██████".to_string(),
                        "██    ".to_string(),
                        "██████".to_string(),
                        "    ██".to_string(),
                        "██████".to_string(),
                    ],
                    vec![
                        "██████".to_string(),
                        "██    ".to_string(),
                        "██████".to_string(),
                        "██  ██".to_string(),
                        "██████".to_string(),
                    ],
                    vec![
                        "██████".to_string(),
                        "    ██".to_string(),
                        "    ██".to_string(),
                        "    ██".to_string(),
                        "    ██".to_string(),
                    ],
                    vec![
                        "██████".to_string(),
                        "██  ██".to_string(),
                        "██████".to_string(),
                        "██  ██".to_string(),
                        "██████".to_string(),
                    ],
                    vec![
                        "██████".to_string(),
                        "██  ██".to_string(),
                        "██████".to_string(),
                        "    ██".to_string(),
                        "██████".to_string(),
                    ],
                ],
                colon: Colon {
                    width: 4,
                    lines: vec![
                        "    ".to_string(),
                        " ██ ".to_string(),
                        "    ".to_string(),
                        " ██ ".to_string(),
                        "    ".to_string(),
                    ],
                },
            },
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

        let meridiem = self.general.merdiem.clone();

        Ok((meridiem, self.into(), font))
    }
}
