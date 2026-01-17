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

#[derive(Debug, Clone, Copy, Default)]
pub enum TextFont {
    #[default]
    NotosansRegular,
    NotosansItalic,
    NotosansMono,
    NotosansBold,
    NotosansBoldItalic,
}
