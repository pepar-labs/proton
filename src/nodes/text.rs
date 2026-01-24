use crate::nodes::Node;
use crate::style::{Color, TextAlign, TextOverflow, TextWrap};
use crate::TextFont;

#[derive(Debug, Clone)]
pub struct TextNode {
    pub content: String,
    pub font_size: f32,
    pub color: Color,
    pub wrap: TextWrap,
    pub overflow: TextOverflow,
    pub align: TextAlign,
    pub font: TextFont,
}

impl Default for TextNode {
    fn default() -> Self {
        Self {
            content: String::new(),
            font_size: 24.0,
            color: Color::Black,
            wrap: TextWrap::None,
            overflow: TextOverflow::Clip,
            align: TextAlign::Left,
            font: TextFont::NotosansRegular,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Text {
    node: TextNode,
}

#[derive(Debug, Clone)]
pub struct TextSpan {
    pub content: String,
    pub font_size: f32,
    pub color: Color,
    pub wrap: TextWrap,
    pub overflow: TextOverflow,
    pub align: TextAlign,
    pub font: TextFont,
}

impl Default for TextSpan {
    fn default() -> Self {
        Self {
            content: String::new(),
            font_size: 24.0,
            color: Color::Black,
            wrap: TextWrap::Word,
            overflow: TextOverflow::Clip,
            align: TextAlign::Left,
            font: TextFont::NotosansRegular,
        }
    }
}

impl Text {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            node: TextNode {
                content: content.into(),
                ..Default::default()
            },
        }
    }

    pub fn size(mut self, px: f32) -> Self {
        self.node.font_size = px;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.node.color = color;
        self
    }

    pub fn wrap(mut self, wrap: TextWrap) -> Self {
        self.node.wrap = wrap;
        self
    }

    pub fn overflow(mut self, overflow: TextOverflow) -> Self {
        self.node.overflow = overflow;
        self
    }

    pub fn align(mut self, align: TextAlign) -> Self {
        self.node.align = align;
        self
    }

    pub fn font(mut self, font: TextFont) -> Self {
        self.node.font = font;
        self
    }

    pub fn bold(mut self) -> Self {
        self.node.font = TextFont::NotosansBold;
        self
    }

    pub fn italic(mut self) -> Self {
        self.node.font = TextFont::NotosansItalic;
        self
    }

    pub fn mono(mut self) -> Self {
        self.node.font = TextFont::NotosansMono;
        self
    }

    pub fn build(self) -> Node {
        Node::Text(self.node)
    }
}

impl From<Text> for Node {
    fn from(builder: Text) -> Node {
        builder.build()
    }
}
