use ab_glyph::FontRef;

use crate::style::TextFont;

use super::data::{
    NOTOSANS_BOLD, NOTOSANS_BOLDITALIC, NOTOSANS_ITALIC, NOTOSANS_MONO, NOTOSANS_REGULAR,
};

#[derive(Debug, Clone)]
pub struct Fonts {
    pub noto_sans_regular: FontRef<'static>,
    pub noto_sans_italic: FontRef<'static>,
    pub noto_sans_mono: FontRef<'static>,
    pub noto_sans_bold: FontRef<'static>,
    pub noto_sans_bold_italic: FontRef<'static>,
}

impl Fonts {
    pub fn new() -> Self {
        Self {
            noto_sans_regular: FontRef::try_from_slice(NOTOSANS_REGULAR)
                .expect("failed to load noto sans regular font"),
            noto_sans_italic: FontRef::try_from_slice(NOTOSANS_ITALIC)
                .expect("failed to load noto sans italic font"),
            noto_sans_bold_italic: FontRef::try_from_slice(NOTOSANS_BOLDITALIC)
                .expect("failed to load noto sans bold-italic font"),
            noto_sans_mono: FontRef::try_from_slice(NOTOSANS_MONO)
                .expect("failed to load noto sans mono font"),
            noto_sans_bold: FontRef::try_from_slice(NOTOSANS_BOLD)
                .expect("failed to load noto sans bold font"),
        }
    }

    pub fn get(&self, font: TextFont) -> &FontRef<'static> {
        match font {
            TextFont::NotosansRegular => &self.noto_sans_regular,
            TextFont::NotosansItalic => &self.noto_sans_italic,
            TextFont::NotosansBold => &self.noto_sans_bold,
            TextFont::NotosansMono => &self.noto_sans_mono,
            TextFont::NotosansBoldItalic => &self.noto_sans_bold_italic,
        }
    }
}

impl Default for Fonts {
    fn default() -> Self {
        Self::new()
    }
}
