use unicode_width::UnicodeWidthStr;

use super::SerializeFont as ConfigFont;
use crate::font::{Colon as EngineColon, Font as EngineFont};

#[derive(Debug, thiserror::Error)]
pub enum ParseConfigFontErr {
    #[error("Width of `{name}` digit is incorrect (failed at line {line}; expected: {width}).")]
    WidthIncorrect {
        name: &'static str,
        line: usize,
        width: u16,
    },
    #[error("Height of `{name}` digit is incorrect (expected: {height}).")]
    HeightIncorrect { name: &'static str, height: u16 },
    #[error("`{0}` digit is empty!")]
    EmptyDigit(&'static str),
}

impl TryFrom<ConfigFont> for EngineFont {
    type Error = ParseConfigFontErr;

    fn try_from(value: ConfigFont) -> Result<Self, Self::Error> {
        value.checker()?;

        Ok(Self {
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
            ],
            height: value.height,
            width: value.width_number,
            colon: EngineColon {
                width: value.width_colon,
                lines: value.colon,
            },
        })
    }
}

impl ConfigFont {
    pub fn checker(&self) -> Result<(), ParseConfigFontErr> {
        let all_digits = [
            ("0", &self.zero, &self.width_number),
            ("1", &self.one, &self.width_number),
            ("2", &self.two, &self.width_number),
            ("3", &self.three, &self.width_number),
            ("4", &self.four, &self.width_number),
            ("5", &self.five, &self.width_number),
            ("6", &self.six, &self.width_number),
            ("7", &self.seven, &self.width_number),
            ("8", &self.eight, &self.width_number),
            ("9", &self.nine, &self.width_number),
            (":", &self.colon, &self.width_colon),
        ];

        for (name, lines, expected_width) in all_digits {
            // 1. Check empty
            if lines.is_empty() {
                return Err(ParseConfigFontErr::EmptyDigit(name));
            }

            // 2. Check height
            if lines.len() != self.height as usize {
                return Err(ParseConfigFontErr::HeightIncorrect {
                    name,
                    height: self.height,
                });
            }

            // 3. Check width
            for (i, line) in lines.iter().enumerate() {
                if line.width() != *expected_width as usize {
                    return Err(ParseConfigFontErr::WidthIncorrect {
                        name,
                        line: i,
                        width: *expected_width,
                    });
                }
            }
        }
        Ok(())
    }
}
