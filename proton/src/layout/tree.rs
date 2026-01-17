use crate::style::Rect;

use super::node_data::NodeData;

#[derive(Debug, Clone)]
pub struct LayoutTree {
    pub nodes: Vec<LayoutNode>,
}

#[derive(Debug, Clone)]
pub struct LayoutNode {
    pub rect: Rect,
    pub data: Option<NodeData>,
}
