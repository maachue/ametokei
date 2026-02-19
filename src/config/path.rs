use std::path::PathBuf;

pub fn get_config_file() -> Option<PathBuf> {
    directories::ProjectDirs::from("com", "maachue", "tty-clock-rs")
        .map(|p| p.config_dir().to_path_buf().join("config.toml"))
}
