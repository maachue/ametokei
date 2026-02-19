pub struct Colon {
    pub width: u16,
    pub lines: Vec<String>,
}

pub struct Fonts {
    pub digits: [Vec<String>; 10],
    pub width: u16,
    pub height: u16,
    pub colon: Colon,
}
