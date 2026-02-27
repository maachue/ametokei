use super::{RuntimeConfig, user::UserConfig};

impl From<UserConfig> for RuntimeConfig {
    fn from(value: UserConfig) -> Self {
        let format_date = if !value.general.hide_date {
            Some(value.general.format_date)
        } else {
            None
        };

        Self {
            show_seconds: value.general.show_seconds,
            blink_colon: value.general.blink_colon,
            color: value.general.color,
            fps: value.performance.fps,
            tps: value.performance.tps,
            center: value.general.center,
            hour12h: value.general.hour12h,
            format_date,
            utc: value.general.utc,
            meridiem: value.general.meridiem,
            spacing: (
                value.fontconfig.spacing_horizontal,
                value.fontconfig.spacing_vertical,
            ),
            timer_mode: value.general.timer_mode.map(|e| e.into()),
        }
    }
}