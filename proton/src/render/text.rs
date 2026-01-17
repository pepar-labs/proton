use ab_glyph::{point, Font, FontRef, ScaleFont};

use crate::font::Fonts;
use crate::layout::LayoutNode;
use crate::nodes::TextNode;
use crate::style::{Rect, TextAlign, TextOverflow};
use crate::text::wrap_text;

use super::primitives::is_within_clip;
use super::target::RenderTarget;

/// Render text to a target.
pub fn render_text<T: RenderTarget>(
    target: &mut T,
    fonts: &Fonts,
    text: &TextNode,
    layout_node: &LayoutNode,
    clip: Option<&Rect>,
) {
    let font = fonts.get(text.font);
    let rect = &layout_node.rect;
    let scaled_font = font.as_scaled(text.font_size);
    let line_height = scaled_font.height();
    let ascent = scaled_font.ascent();
    let luma = text.color.to_luma();

    let lines = wrap_text(font, &text.content, text.font_size, rect.width, text.wrap);

    let max_lines = (rect.height / line_height).floor() as usize;
    let max_lines = max_lines.max(1);

    let needs_ellipsis = text.overflow == TextOverflow::Ellipsis && lines.len() > max_lines;

    let visible_count = lines.len().min(max_lines);

    for (line_idx, line) in lines.iter().take(visible_count).enumerate() {
        let is_last_visible = line_idx == visible_count - 1;

        let line_to_render = if is_last_visible && needs_ellipsis {
            truncate_with_ellipsis(font, line, text.font_size, rect.width)
        } else {
            line.clone()
        };

        let line_width = measure_line(font, &line_to_render, text.font_size);

        let x_offset = match text.align {
            TextAlign::Left => 0.0,
            TextAlign::Center => (rect.width - line_width) / 2.0,
            TextAlign::Right => rect.width - line_width,
        };

        let baseline_y = rect.y + ascent + (line_idx as f32 * line_height);
        let start_x = rect.x + x_offset;

        render_line_clipped(
            target,
            font,
            &line_to_render,
            start_x,
            baseline_y,
            text.font_size,
            luma,
            clip,
        );
    }
}

fn measure_line(font: &FontRef<'static>, text: &str, font_size: f32) -> f32 {
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

fn render_line_clipped<T: RenderTarget>(
    target: &mut T,
    font: &FontRef<'static>,
    text: &str,
    start_x: f32,
    baseline_y: f32,
    font_size: f32,
    luma: u8,
    clip: Option<&Rect>,
) {
    let scaled_font = font.as_scaled(font_size);
    let mut cursor_x = start_x;
    let mut prev_glyph: Option<ab_glyph::GlyphId> = None;

    for ch in text.chars() {
        let glyph_id = font.glyph_id(ch);

        if let Some(prev) = prev_glyph {
            cursor_x += scaled_font.kern(prev, glyph_id);
        }

        let glyph = glyph_id.with_scale_and_position(font_size, point(cursor_x, baseline_y));

        if let Some(outlined) = font.outline_glyph(glyph) {
            let bounds = outlined.px_bounds();
            outlined.draw(|px, py, coverage| {
                let x = bounds.min.x as i32 + px as i32;
                let y = bounds.min.y as i32 + py as i32;

                if is_within_clip(x, y, clip) {
                    let existing = target.get_pixel(x, y);
                    let blended =
                        ((1.0 - coverage) * existing as f32 + coverage * luma as f32) as u8;
                    target.set_pixel(x, y, blended);
                }
            });
        }

        cursor_x += scaled_font.h_advance(glyph_id);
        prev_glyph = Some(glyph_id);
    }
}

fn truncate_with_ellipsis(
    font: &FontRef<'static>,
    text: &str,
    font_size: f32,
    max_width: f32,
) -> String {
    let ellipsis = "...";
    let ellipsis_width = measure_line(font, ellipsis, font_size);
    let available_width = max_width - ellipsis_width;

    if available_width <= 0.0 {
        return ellipsis.to_string();
    }

    let mut result = String::new();
    let mut current_width = 0.0f32;
    let scaled_font = font.as_scaled(font_size);

    for ch in text.chars() {
        let glyph_id = font.glyph_id(ch);
        let char_width = scaled_font.h_advance(glyph_id);

        if current_width + char_width > available_width {
            break;
        }

        result.push(ch);
        current_width += char_width;
    }

    result.push_str(ellipsis);
    result
}
