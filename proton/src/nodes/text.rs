use crate::{nodes::Node, Color};

#[derive(Debug, Clone)]
pub struct TextNode {
    pub content: String,
    pub font_size: f32,
    pub color: Color,
}

impl Default for TextNode {
    fn default() -> Self {
        Self {
            content: String::new(),
            font_size: 24.0,
            color: Color::Black,
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

    pub fn build(self) -> Node {
        Node::Text(self.node)
    }
}

impl From<Text> for Node {
    fn from(builder: Text) -> Node {
        builder.build()
    }
}
