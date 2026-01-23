#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Color {
    #[default]
    White,
    Black,
    // 8 bit grayscale
    // 0->black, 255->white
    Gray(u8),
}

impl Color {
    pub fn to_luma(&self) -> u8 {
        match self {
            Color::White => 255,
            Color::Black => 0,
            Color::Gray(v) => *v,
        }
    }
}
