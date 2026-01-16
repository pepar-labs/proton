use crate::{nodes::Node, Align, Color, Dimension, FlexDirection, Justify};

#[derive(Debug, Clone)]
pub struct ViewNode {
    pub children: Vec<Node>,
    pub direction: FlexDirection,
    pub justify: Justify,
    pub align: Align,
    pub padding: f32,
    pub gap: f32,
    pub background: Option<Color>,
    pub width: Dimension,
    pub height: Dimension,
}

impl Default for ViewNode {
    fn default() -> Self {
        Self {
            children: Vec::new(),
            direction: FlexDirection::Column,
            justify: Justify::Start,
            align: Align::Stretch,
            padding: 0.0,
            gap: 0.0,
            background: None,
            width: Dimension::Auto,
            height: Dimension::Auto,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct View {
    node: ViewNode,
}

impl View {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn column() -> Self {
        Self {
            node: ViewNode {
                direction: FlexDirection::Column,
                ..Default::default()
            },
        }
    }

    pub fn row() -> Self {
        Self {
            node: ViewNode {
                direction: FlexDirection::Row,
                ..Default::default()
            },
        }
    }

    pub fn direction(mut self, direction: FlexDirection) -> Self {
        self.node.direction = direction;
        self
    }

    pub fn justify(mut self, justify: Justify) -> Self {
        self.node.justify = justify;
        self
    }

    pub fn align(mut self, align: Align) -> Self {
        self.node.align = align;
        self
    }

    pub fn padding(mut self, px: f32) -> Self {
        self.node.padding = px;
        self
    }

    pub fn gap(mut self, px: f32) -> Self {
        self.node.gap = px;
        self
    }

    pub fn background(mut self, color: Color) -> Self {
        self.node.background = Some(color);
        self
    }

    pub fn width(mut self, dim: Dimension) -> Self {
        self.node.width = dim;
        self
    }

    pub fn height(mut self, dim: Dimension) -> Self {
        self.node.height = dim;
        self
    }

    pub fn child(mut self, node: impl Into<Node>) -> Self {
        self.node.children.push(node.into());
        self
    }

    pub fn children(mut self, nodes: impl IntoIterator<Item = impl Into<Node>>) -> Self {
        for node in nodes {
            self.node.children.push(node.into());
        }
        self
    }

    pub fn build(self) -> Node {
        Node::View(self.node)
    }
}

impl From<View> for Node {
    fn from(builder: View) -> Node {
        builder.build()
    }
}
