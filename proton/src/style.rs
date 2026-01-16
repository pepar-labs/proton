//! Style types for Proton UI components

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
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

impl Default for Color {
    fn default() -> Self {
        Color::White
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Dimension {
    #[default]
    Auto,
    Px(f32),
    // parent's percent * 100
    Percent(f32),
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum FlexDirection {
    Row,
    #[default]
    Column,
}

// main axis alignment
// if column -> y axis
// if row -> x axis
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Justify {
    #[default]
    Start,
    End,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

// cross axis alignment
// if column -> x axis
// if row -> y axis
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Align {
    Start,
    End,
    Center,
    #[default]
    Stretch,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ImageFit {
    #[default]
    Contain,
    Cover,
    Fill,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum TextWrap {
    #[default]
    None,
    Word,
    Char,
    WordChar,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum TextOverflow {
    #[default]
    Clip,
    Ellipsis,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum TextAlign {
    #[default]
    Left,
    Center,
    Right,
}
