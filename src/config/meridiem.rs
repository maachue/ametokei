use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MeridiemConfig {
    pub am: String,
    pub pm: String,
}
impl Default for MeridiemConfig {
    fn default() -> Self {
        Self {
            am: " [AM]".to_string(),
            pm: " [PM]".to_string(),
        }
    }
}
impl MeridiemConfig {
    pub fn get(&self, is_pm: bool) -> &str {
        match is_pm {
            false => &self.am,
            true => &self.pm,
        }
    }
}
