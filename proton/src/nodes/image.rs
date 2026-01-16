use crate::{
    nodes::Node,
    style::{Dimension, ImageFit},
};

#[derive(Debug, Clone)]
pub enum ImageSource {
    Path(String),
    Bytes(Vec<u8>),
}

#[derive(Debug, Clone)]
pub struct ImageNode {
    pub source: ImageSource,
    pub width: Dimension,
    pub height: Dimension,
    pub fit: ImageFit,
}

impl Default for ImageNode {
    fn default() -> Self {
        Self {
            source: ImageSource::Path(String::new()),
            width: Dimension::Auto,
            height: Dimension::Auto,
            fit: ImageFit::Contain,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Image {
    node: ImageNode,
}

impl Image {
    pub fn from_path(path: impl Into<String>) -> Self {
        Self {
            node: ImageNode {
                source: ImageSource::Path(path.into()),
                ..Default::default()
            },
        }
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self {
            node: ImageNode {
                source: ImageSource::Bytes(bytes),
                ..Default::default()
            },
        }
    }

    pub fn width(mut self, dim: Dimension) -> Self {
        self.node.width = dim;
        self
    }

    pub fn height(mut self, dim: Dimension) -> Self {
        self.node.height = dim;
        self
    }

    pub fn fit(mut self, fit: ImageFit) -> Self {
        self.node.fit = fit;
        self
    }

    pub fn build(self) -> Node {
        Node::Image(self.node)
    }
}

impl From<Image> for Node {
    fn from(builder: Image) -> Self {
        builder.build()
    }
}
