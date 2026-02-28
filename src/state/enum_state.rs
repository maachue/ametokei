#[derive(Copy, Clone, PartialEq, Eq)]
pub enum EnoughSize {
    Enough,
    Not(u16, u16),
}
impl EnoughSize {
    pub fn or(self, sr: Self) -> Self {
        match sr {
            Self::Not(_, _) => sr,
            Self::Enough => self,
        }
    }

    pub fn is_enough(&self) -> bool {
        *self == Self::Enough
    }
}