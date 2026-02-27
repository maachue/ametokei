use std::collections::HashMap;

use color_eyre::eyre::Result;

use crate::config::user::SerializeFont;

pub struct Font {
    pub width: u16,
    pub colon_width: u16,
    pub height: u16,

    pub symbols: [char; 5],
    pub digits: [Vec<u8>; 11],
}
impl Font {
    pub fn tenki() -> Self {
        Self {
            width: 5,
            height: 5,
            colon_width: 1,
            symbols: ['█', '▀', char::default(), char::default(), char::default()],
            #[rustfmt::skip]
            digits: [
                vec![
                    1, 1, 1, 1, 1,
                    1, 1, 0, 1, 1,
                    1, 1, 0, 1, 1,
                    1, 1, 0, 1, 1,
                    1, 1, 1, 1, 1
                ],
                vec![
                    0, 0, 0, 1, 1,
                    0, 0, 0, 1, 1,
                    0, 0, 0, 1, 1,
                    0, 0, 0, 1, 1,
                    0, 0, 0, 1, 1
                ],
                vec![
                    1, 1, 1, 1, 1,
                    0, 0, 0, 1, 1,
                    1, 1, 1, 1, 1,
                    1, 1, 0, 0, 0,
                    1, 1, 1, 1, 1
                ],
                vec![
                    1, 1, 1, 1, 1,
                    0, 0, 0, 1, 1,
                    1, 1, 1, 1, 1,
                    0, 0, 0, 1, 1,
                    1, 1, 1, 1, 1
                ],
                vec![
                    1, 1, 0, 1, 1,
                    1, 1, 0, 1, 1,
                    1, 1, 1, 1, 1,
                    0, 0, 0, 1, 1,
                    0, 0, 0, 1, 1
                ],
                vec![
                    1, 1, 1, 1, 1,
                    1, 1, 0, 0, 0,
                    1, 1, 1, 1, 1,
                    0, 0, 0, 1, 1,
                    1, 1, 1, 1, 1
                ],
                vec![
                    1, 1, 1, 1, 1,
                    1, 1, 0, 0, 0,
                    1, 1, 1, 1, 1,
                    1, 1, 0, 1, 1,
                    1, 1, 1, 1, 1
                ],
                vec![
                    1, 1, 1, 1, 1,
                    1, 1, 0, 1, 1,
                    0, 0, 0, 1, 1,
                    0, 0, 0, 1, 1,
                    0, 0, 0, 1, 1
                ],
                vec![
                    1, 1, 1, 1, 1,
                    1, 1, 0, 1, 1,
                    1, 1, 1, 1, 1,
                    1, 1, 0, 1, 1,
                    1, 1, 1, 1, 1
                ],
                vec![
                    1, 1, 1, 1, 1,
                    1, 1, 0, 1, 1,
                    1, 1, 1, 1, 1,
                    0, 0, 0, 1, 1,
                    1, 1, 1, 1, 1
                ],
                vec![
                          0,
                          2,
                          0,
                          2,
                          0
                ],
            ],
        }
    }

    pub fn digital() -> Self {
        Self {
            width: 6,
            height: 5,
            colon_width: 4,
            symbols: [
                '█',
                char::default(),
                char::default(),
                char::default(),
                char::default(),
            ],
            #[rustfmt::skip]
            digits: [
                vec![
                    1, 1, 1, 1, 1, 1,
                    1, 1, 0, 0, 1, 1,
                    1, 1, 0, 0, 1, 1,
                    1, 1, 0, 0, 1, 1,
                    1, 1, 1, 1, 1, 1
                ],
                vec![
                    0, 0, 0, 0, 1, 1,
                    0, 0, 0, 0, 1, 1,
                    0, 0, 0, 0, 1, 1,
                    0, 0, 0, 0, 1, 1,
                    0, 0, 0, 0, 1, 1
                ],
                vec![
                    1, 1, 1, 1, 1, 1,
                    0, 0, 0, 0, 1, 1,
                    1, 1, 1, 1, 1, 1,
                    1, 1, 0, 0, 0, 0,
                    1, 1, 1, 1, 1, 1
                ],
                vec![
                    1, 1, 1, 1, 1, 1,
                    0, 0, 0, 0, 1, 1,
                    1, 1, 1, 1, 1, 1,
                    0, 0, 0, 0, 1, 1,
                    1, 1, 1, 1, 1, 1
                ],
                vec![
                    1, 1, 0, 0, 1, 1,
                    1, 1, 0, 0, 1, 1,
                    1, 1, 1, 1, 1, 1,
                    0, 0, 0, 0, 1, 1,
                    0, 0, 0, 0, 1, 1
                ],
                vec![
                    1, 1, 1, 1, 1, 1,
                    0, 0, 0, 0, 1, 1,
                    1, 1, 1, 1, 1, 1,
                    1, 1, 0, 0, 0, 0,
                    1, 1, 1, 1, 1, 1
                ],
                vec![
                    1, 1, 1, 1, 1, 1,
                    1, 1, 0, 0, 0, 0,
                    1, 1, 1, 1, 1, 1,
                    1, 1, 0, 0, 1, 1,
                    1, 1, 1, 1, 1, 1
                ],
                vec![
                    1, 1, 1, 1, 1, 1,
                    0, 0, 0, 0, 1, 1,
                    0, 0, 0, 0, 1, 1,
                    0, 0, 0, 0, 1, 1,
                    0, 0, 0, 0, 1, 1
                ],
                vec![
                    1, 1, 1, 1, 1, 1,
                    1, 1, 0, 0, 1, 1,
                    1, 1, 1, 1, 1, 1,
                    1, 1, 0, 0, 1, 1,
                    1, 1, 1, 1, 1, 1
                ],
                vec![
                    1, 1, 1, 1, 1, 1,
                    1, 1, 0, 0, 1, 1,
                    1, 1, 1, 1, 1, 1,
                    0, 0, 0, 0, 1, 1,
                    1, 1, 1, 1, 1, 1
                ],
                vec![
                       0, 0, 0, 0,
                       0, 1, 1, 0,
                       0, 0, 0, 0,
                       0, 1, 1, 0,
                       0, 0, 0, 0
                ],
            ],
        }
    }

    pub fn get(name: &str, mut fonts: Option<&mut HashMap<String, SerializeFont>>) -> Result<Self> {
        match name {
            "digital" => Ok(Self::digital()),
            "tenki" => Ok(Self::tenki()),
            _ => {
                use color_eyre::eyre::OptionExt;

                let fonts = fonts.take().ok_or_eyre("User fonts are not defined.")?;
                let user_font = fonts.remove(name).ok_or_eyre(format!("Font `{}` not found", name))?;

                Ok(user_font.try_into()?)
            }
        }
    }
}