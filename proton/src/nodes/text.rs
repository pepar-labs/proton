use crate::nodes::Node;
use crate::style::{Color, TextAlign, TextOverflow, TextWrap};

#[derive(Debug, Clone)]
pub struct TextNode {
    pub content: String,
    pub font_size: f32,
    pub color: Color,
    pub wrap: TextWrap,
    pub overflow: TextOverflow,
    pub align: TextAlign,
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
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Text {
    node: TextNode,
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

    pub fn build(self) -> Node {
        Node::Text(self.node)
    }
}

impl From<Text> for Node {
    fn from(builder: Text) -> Node {
        builder.build()
    }
}
