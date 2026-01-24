
use ab_glyph::FontRef;

use super::measure::line_height;
use super::wrap::wrap_text;
use crate::style::TextWrap;

#[derive(Debug, Clone)]
pub struct TextPaginator {
    pages: Vec<String>,
}

impl TextPaginator {
    pub fn new(
        font: &FontRef<'static>,
        content: &str,
        font_size: f32,
        available_width: f32,
        available_height: f32,
        wrap: TextWrap,
    ) -> Self {
        let pages = paginate_text(font, content, font_size, available_width, available_height, wrap);
        Self { pages }
    }

    pub fn page_count(&self) -> usize {
        self.pages.len()
    }

    pub fn get_page(&self, index: usize) -> Option<&str> {
        self.pages.get(index).map(|s| s.as_str())
    }

    pub fn pages(&self) -> &[String] {
        &self.pages
    }

    pub fn is_empty(&self) -> bool {
        self.pages.is_empty()
    }
}

fn paginate_text(
    font: &FontRef<'static>,
    content: &str,
    font_size: f32,
    available_width: f32,
    available_height: f32,
    wrap: TextWrap,
) -> Vec<String> {
    let lines = wrap_text(font, content, font_size, available_width, wrap);
    
    if lines.is_empty() {
        return vec![String::new()];
    }

    let line_h = line_height(font, font_size);
    let lines_per_page = (available_height / line_h).floor() as usize;
    
    if lines_per_page == 0 {
        return vec![content.to_string()];
    }

    let mut pages = Vec::new();
    let mut current_page_lines: Vec<&str> = Vec::new();

    for line in &lines {
        current_page_lines.push(line);
        
        if current_page_lines.len() >= lines_per_page {
            pages.push(current_page_lines.join("\n"));
            current_page_lines.clear();
        }
    }

    if !current_page_lines.is_empty() {
        pages.push(current_page_lines.join("\n"));
    }

    if pages.is_empty() {
        pages.push(String::new());
    }

    pages
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::font::Fonts;
    use crate::style::TextFont;

    #[test]
    fn test_paginator_basic() {
        let fonts = Fonts::new();
        let font = fonts.get(TextFont::NotosansRegular);
        
        let content = "Line 1\nLine 2\nLine 3\nLine 4\nLine 5";
        let paginator = TextPaginator::new(
            font,
            content,
            24.0,
            400.0,
            50.0, 
            TextWrap::Word,
        );
        
        assert!(paginator.page_count() > 0);
    }

    #[test]
    fn test_paginator_empty_content() {
        let fonts = Fonts::new();
        let font = fonts.get(TextFont::NotosansRegular);
        
        let paginator = TextPaginator::new(
            font,
            "",
            24.0,
            400.0,
            400.0,
            TextWrap::Word,
        );
        
        assert_eq!(paginator.page_count(), 1);
        assert_eq!(paginator.get_page(0), Some(""));
    }
}
