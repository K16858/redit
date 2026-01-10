#[derive(Copy, Clone)]
pub enum GraphemeWidth {
    Half,
    Full,
}

impl GraphemeWidth {
    pub fn saturating_add(self, other: usize) -> usize {
        match self {
            Self::Half => other.saturating_add(1),
            Self::Full => other.saturating_add(2),
        }
    }
}
