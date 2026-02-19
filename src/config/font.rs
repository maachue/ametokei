use unicode_width::UnicodeWidthStr;

use super::SerializeFont as ConfigFont;
use crate::font::{Colon as EngineColon, Fonts as EngineFonts};

#[derive(Debug, thiserror::Error)]
pub enum ParseConfigFontErr {
    #[error("Width of `{name}` digit is incorrect (failed at line {line}; expected: {width}).")]
    WidthIncorrect {
        name: String,
        line: usize,
        width: u16,
    },
    #[error("Height of `{name}` digit is incorrect (expected: {height}).")]
    HeightIncorrect { name: String, height: u16 },
    #[error("`{0}` digit is empty!")]
    EmptyDigit(String),
}

pub fn check_width_in_digit(
    digit: &[String],
    expected: usize,
) -> Result<(), usize /* line cause error */> {
    let mut len = digit.first().unwrap().len();

    if len != expected {
        return Err(1);
    }

    for (i, line) in digit.iter().enumerate() {
        let new_len = line.len();
        if new_len == len {
            len = new_len
        } else {
            return Err(i);
        }
    }

    Ok(())
}

impl TryFrom<ConfigFont> for EngineFonts {
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
            ("0", &self.zero),
            ("1", &self.one),
            ("2", &self.two),
            ("3", &self.three),
            ("4", &self.four),
            ("5", &self.five),
            ("6", &self.six),
            ("7", &self.seven),
            ("8", &self.eight),
            ("9", &self.nine),
            (":", &self.colon),
        ];

        for (name, lines) in all_digits {
            let expected_width = if name == ":" {
                self.width_colon
            } else {
                self.width_number
            } as usize;

            // 1. Check empty
            if lines.is_empty() {
                return Err(ParseConfigFontErr::EmptyDigit(name.to_string()));
            }

            // 2. Check height
            if lines.len() != self.height as usize {
                return Err(ParseConfigFontErr::HeightIncorrect {
                    name: name.to_string(),
                    height: self.height,
                });
            }

            // 3. Check width
            for (i, line) in lines.iter().enumerate() {
                if line.width() != expected_width {
                    return Err(ParseConfigFontErr::WidthIncorrect {
                        name: name.to_string(),
                        line: i,
                        width: expected_width as u16,
                    });
                }
            }
        }
        Ok(())
    }
}
