
use crate::{nodes::Node, Align, Color, Dimension, FlexDirection, Justify};

#[derive(Debug, Clone)]
pub struct ListItem {
    pub node: Node,
    pub id: u32,
}

#[derive(Debug, Clone)]
pub struct ListViewNode {
    pub children: Vec<Node>,
    pub direction: FlexDirection,
    pub justify: Justify,
    pub align: Align,
    pub padding: f32,
    pub gap: f32,
    pub background: Option<Color>,
    pub width: Dimension,
    pub height: Dimension,
    pub selected_index: Option<usize>,
    pub scroll_offset: f32,
    pub selected_background: Color,
}

impl Default for ListViewNode {
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
            selected_index: None,
            scroll_offset: 0.0,
            selected_background: Color::Gray(220),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ListView {
    node: ListViewNode,
}

impl ListView {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn vertical() -> Self {
        Self {
            node: ListViewNode {
                direction: FlexDirection::Column,
                ..Default::default()
            },
        }
    }

    pub fn horizontal() -> Self {
        Self {
            node: ListViewNode {
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

    pub fn selected_index(mut self, index: Option<usize>) -> Self {
        self.node.selected_index = index;
        self
    }

    pub fn scroll_offset(mut self, offset: f32) -> Self {
        self.node.scroll_offset = offset.max(0.0);
        self
    }

    pub fn selected_background(mut self, color: Color) -> Self {
        self.node.selected_background = color;
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
        Node::ListView(self.node)
    }
}

impl From<ListView> for Node {
    fn from(builder: ListView) -> Node {
        builder.build()
    }
}
