use super::user::SerializeFont;
use crate::font::Font;

#[derive(Debug, thiserror::Error)]
pub enum ParseConfigFontError {
    #[error("Size of `{name}` digit is incorrect (expected: {width}*{height}).")]
    SizeIncorrect {
        name: &'static str,
        width: u16,
        height: u16,
    },

    #[error("`{0}` digit is empty!")]
    EmptyDigit(&'static str),

    #[error("The `{name}` digit contains invalid/undefined symbols/characters (index char: {idx}; index array: {idx_arr}).")]
    InvalidChar { name: &'static str, idx: u8, idx_arr: usize },

    #[error(
        "The `{name}` digit contains a bit that is out of bounds (allowed: 0..={max}, found: {used})."
    )]
    UseOutOfBoundChar {
        name: &'static str,
        max: u8,
        used: u8,
    },
}

impl TryFrom<SerializeFont> for Font {
    type Error = ParseConfigFontError;

    fn try_from(value: SerializeFont) -> Result<Self, Self::Error> {
        value.checker()?;

        Ok(Self {
            width: value.num_width,
            colon_width: value.colon_width,
            height: value.height,
            first_sym: value.symbol_1,
            symbols: [
                value.symbol_2,
                value.symbol_3,
                value.symbol_4,
                value.symbol_5,
            ],
            digits: [
                value.zero,
                value.one,
                value.two,
                value.three,
                value.four,
                value.five,
                value.six,
                value.seven,
                value.eight,
                value.nine,
                value.colon,
            ],
        })
    }
}

impl SerializeFont {
    pub fn checker(&self) -> Result<(), ParseConfigFontError> {
        let symbols = [
            Some(&self.symbol_1),
            self.symbol_2.as_ref(),
            self.symbol_3.as_ref(),
            self.symbol_4.as_ref(),
            self.symbol_5.as_ref(),
        ];

        let max = 5;
        let all_digits = [
            ("0", &self.zero, &self.num_width),
            ("1", &self.one, &self.num_width),
            ("2", &self.two, &self.num_width),
            ("3", &self.three, &self.num_width),
            ("4", &self.four, &self.num_width),
            ("5", &self.five, &self.num_width),
            ("6", &self.six, &self.num_width),
            ("7", &self.seven, &self.num_width),
            ("8", &self.eight, &self.num_width),
            ("9", &self.nine, &self.num_width),
            (":", &self.colon, &self.colon_width),
        ];

        for (name, digit, expected_width) in all_digits {
            // 1. check empty
            if digit.is_empty() {
                return Err(ParseConfigFontError::EmptyDigit(name));
            }

            // 2. check size
            if digit.len() != (*expected_width as usize) * (self.height as usize) {
                return Err(ParseConfigFontError::SizeIncorrect {
                    name,
                    width: *expected_width,
                    height: self.height,
                });
            }

            for (i, bit) in digit.iter().enumerate() {
                // 0 is space, always valid
                if *bit == 0 {
                    continue;
                }

                // out of bounds check
                if *bit > max {
                    return Err(ParseConfigFontError::UseOutOfBoundChar {
                        name,
                        max,
                        used: *bit,
                    });
                }

                if symbols[(*bit - 1) as usize].is_none() {
                    return Err(ParseConfigFontError::InvalidChar { name, idx: *bit, idx_arr: i });
                }
            }
        }

        Ok(())
    }
}
