use ab_glyph::{Font, FontRef, ScaleFont};

pub fn measure_text_width(font: &FontRef<'static>, text: &str, font_size: f32) -> f32 {
    let scaled_font = font.as_scaled(font_size);
    let mut width = 0.0f32;
    let mut prev_glyph: Option<ab_glyph::GlyphId> = None;

    for ch in text.chars() {
        let glyph_id = font.glyph_id(ch);

        if let Some(prev) = prev_glyph {
            width += scaled_font.kern(prev, glyph_id);
        }

        width += scaled_font.h_advance(glyph_id);
        prev_glyph = Some(glyph_id);
    }

    width
}

pub fn measure_char_width(font: &FontRef<'static>, ch: char, font_size: f32) -> f32 {
    let scaled_font = font.as_scaled(font_size);
    let glyph_id = font.glyph_id(ch);
    scaled_font.h_advance(glyph_id)
}

pub fn line_height(font: &FontRef<'static>, font_size: f32) -> f32 {
    let scaled_font = font.as_scaled(font_size);
    scaled_font.height()
}
