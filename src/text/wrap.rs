use ab_glyph::FontRef;

use crate::style::TextWrap;

use super::measure::{measure_char_width, measure_text_width};

pub fn wrap_text(
    font: &FontRef<'static>,
    text: &str,
    font_size: f32,
    max_width: f32,
    wrap: TextWrap,
) -> Vec<String> {
    match wrap {
        TextWrap::None => vec![text.to_string()],
        TextWrap::Word => wrap_words(font, text, font_size, max_width, false),
        TextWrap::Char => wrap_chars(font, text, font_size, max_width),
        TextWrap::WordChar => wrap_words(font, text, font_size, max_width, true),
    }
}

fn wrap_words(
    font: &FontRef<'static>,
    text: &str,
    font_size: f32,
    max_width: f32,
    break_long: bool,
) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();
    let mut current_width = 0.0f32;
    let space_width = measure_char_width(font, ' ', font_size);

    for word in text.split_whitespace() {
        let word_width = measure_text_width(font, word, font_size);

        let needed_width = if current_line.is_empty() {
            word_width
        } else {
            space_width + word_width
        };

        if current_width + needed_width <= max_width {
            if !current_line.is_empty() {
                current_line.push(' ');
                current_width += space_width;
            }
            current_line.push_str(word);
            current_width += word_width;
        } else if word_width > max_width && break_long {
            if !current_line.is_empty() {
                lines.push(current_line);
                current_line = String::new();
                current_width = 0.0;
            }
            let char_lines = wrap_chars(font, word, font_size, max_width);
            let char_lines_len = char_lines.len();
            for (i, char_line) in char_lines.into_iter().enumerate() {
                if i == char_lines_len - 1 {
                    current_line = char_line;
                    current_width = measure_text_width(font, &current_line, font_size);
                } else {
                    lines.push(char_line);
                }
            }
        } else {
            if !current_line.is_empty() {
                lines.push(current_line);
            }
            current_line = word.to_string();
            current_width = word_width;
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    if lines.is_empty() {
        lines.push(String::new());
    }

    lines
}

fn wrap_chars(font: &FontRef<'static>, text: &str, font_size: f32, max_width: f32) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();
    let mut current_width = 0.0f32;

    for ch in text.chars() {
        let char_width = measure_char_width(font, ch, font_size);

        if current_width + char_width > max_width && !current_line.is_empty() {
            lines.push(current_line);
            current_line = String::new();
            current_width = 0.0;
        }

        current_line.push(ch);
        current_width += char_width;
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    if lines.is_empty() {
        lines.push(String::new());
    }

    lines
}
