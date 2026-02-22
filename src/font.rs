pub struct Colon {
    pub width: u16,
    pub lines: Vec<String>,
}

pub struct Font {
    pub digits: [Vec<String>; 10],
    pub width: u16,
    pub height: u16,
    pub colon: Colon,
}
impl Font {
    pub fn digital() -> Self {
        Self {
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
        }
    }
    pub fn tenki() -> Self {
        Self {
            width: 5,
            height: 5,
            digits: [
                vec![
                    "█████".to_string(),
                    "██ ██".to_string(),
                    "██ ██".to_string(),
                    "██ ██".to_string(),
                    "█████".to_string(),
                ],
                vec![
                    "   ██".to_string(),
                    "   ██".to_string(),
                    "   ██".to_string(),
                    "   ██".to_string(),
                    "   ██".to_string(),
                ],
                vec![
                    "█████".to_string(),
                    "   ██".to_string(),
                    "█████".to_string(),
                    "██   ".to_string(),
                    "█████".to_string(),
                ],
                vec![
                    "█████".to_string(),
                    "   ██".to_string(),
                    "█████".to_string(),
                    "   ██".to_string(),
                    "█████".to_string(),
                ],
                vec![
                    "██ ██".to_string(),
                    "██ ██".to_string(),
                    "█████".to_string(),
                    "   ██".to_string(),
                    "   ██".to_string(),
                ],
                vec![
                    "█████".to_string(),
                    "██   ".to_string(),
                    "█████".to_string(),
                    "   ██".to_string(),
                    "█████".to_string(),
                ],
                vec![
                    "█████".to_string(),
                    "██   ".to_string(),
                    "█████".to_string(),
                    "██ ██".to_string(),
                    "█████".to_string(),
                ],
                vec![
                    "█████".to_string(),
                    "██ ██".to_string(),
                    "   ██".to_string(),
                    "   ██".to_string(),
                    "   ██".to_string(),
                ],
                vec![
                    "█████".to_string(),
                    "██ ██".to_string(),
                    "█████".to_string(),
                    "██ ██".to_string(),
                    "█████".to_string(),
                ],
                vec![
                    "█████".to_string(),
                    "██ ██".to_string(),
                    "█████".to_string(),
                    "   ██".to_string(),
                    "█████".to_string(),
                ],
            ],
            colon: Colon {
                width: 1,
                lines: vec![
                    " ".to_string(),
                    "▀".to_string(),
                    " ".to_string(),
                    "▀".to_string(),
                    " ".to_string(),
                ],
            },
        }
    }
}
