use crate::nodes::{ImageNode, ImageSource, Node, TextNode, ViewNode};
use crate::style::{
    Align as ProtonAlign, Dimension as ProtonDim, FlexDirection as ProtonDir, ImageFit,
    Justify as ProtonJustify, Rect, Size,
};
use ab_glyph::{Font, ScaleFont};
use image::GenericImageView;
use taffy::prelude::*;

pub struct LayoutEngine {
    taffy: TaffyTree<NodeData>,
    font: ab_glyph::FontRef<'static>,
}

#[derive(Debug, Clone)]
pub enum NodeData {
    View,
    Text { content: String, font_size: f32 },
    Image { source: ImageSource, fit: ImageFit },
}

impl LayoutEngine {
    pub fn new() -> Self {
        let font_data: &'static [u8] = include_bytes!("../fonts/NotoSans-Regular.ttf");
        let font =
            ab_glyph::FontRef::try_from_slice(font_data).expect("Failed to load embedded font");

        Self {
            taffy: TaffyTree::new(),
            font,
        }
    }

    pub fn compute(&mut self, root: &Node, available: Size) -> LayoutTree {
        self.taffy.clear();

        let root_id = self.build_taffy_node(root);

        self.taffy
            .compute_layout(
                root_id,
                taffy::Size {
                    width: AvailableSpace::Definite(available.width),
                    height: AvailableSpace::Definite(available.height),
                },
            )
            .expect("Layout computation failed");

        let mut nodes = Vec::new();
        self.extract_layout(root_id, 0.0, 0.0, &mut nodes);

        LayoutTree { nodes }
    }

    fn build_taffy_node(&mut self, node: &Node) -> NodeId {
        match node {
            Node::View(view) => self.build_view_node(view),
            Node::Text(text) => self.build_text_node(text),
            Node::Image(img) => self.build_image_node(img),
        }
    }

    fn build_view_node(&mut self, view: &ViewNode) -> NodeId {
        let child_ids: Vec<NodeId> = view
            .children
            .iter()
            .map(|child| self.build_taffy_node(child))
            .collect();

        let style = Style {
            display: Display::Flex,
            flex_direction: convert_direction(view.direction),
            justify_content: Some(convert_justify(view.justify)),
            align_items: Some(convert_align(view.align)),
            padding: taffy::Rect {
                left: LengthPercentage::Length(view.padding),
                right: LengthPercentage::Length(view.padding),
                top: LengthPercentage::Length(view.padding),
                bottom: LengthPercentage::Length(view.padding),
            },
            gap: taffy::Size {
                width: LengthPercentage::Length(view.gap),
                height: LengthPercentage::Length(view.gap),
            },
            size: taffy::Size {
                width: convert_dimension(view.width),
                height: convert_dimension(view.height),
            },
            ..Default::default()
        };

        self.taffy
            .new_with_children(style, &child_ids)
            .expect("Failed to create view node")
    }

    fn build_text_node(&mut self, text: &TextNode) -> NodeId {
        let (text_width, text_height) = self.measure_text(&text.content, text.font_size);

        let style = Style {
            size: taffy::Size {
                width: Dimension::Length(text_width),
                height: Dimension::Length(text_height),
            },
            ..Default::default()
        };

        self.taffy
            .new_leaf_with_context(
                style,
                NodeData::Text {
                    content: text.content.clone(),
                    font_size: text.font_size,
                },
            )
            .expect("Failed to create text node")
    }

    fn build_image_node(&mut self, img: &ImageNode) -> NodeId {
        let (intrinsic_width, intrinsic_height) = self.get_image_dimensions(&img.source);

        let width = match img.width {
            ProtonDim::Auto => Dimension::Length(intrinsic_width as f32),
            ProtonDim::Px(px) => Dimension::Length(px),
            ProtonDim::Percent(p) => Dimension::Percent(p),
        };

        let height = match img.height {
            ProtonDim::Auto => Dimension::Length(intrinsic_height as f32),
            ProtonDim::Px(px) => Dimension::Length(px),
            ProtonDim::Percent(p) => Dimension::Percent(p),
        };

        let style = Style {
            size: taffy::Size { width, height },
            ..Default::default()
        };

        self.taffy
            .new_leaf_with_context(
                style,
                NodeData::Image {
                    source: img.source.clone(),
                    fit: img.fit,
                },
            )
            .expect("Failed to create image node")
    }

    fn get_image_dimensions(&self, source: &ImageSource) -> (u32, u32) {
        match source {
            ImageSource::Path(path) => {
                if let Ok(img) = image::open(path) {
                    img.dimensions()
                } else {
                    (100, 100)
                }
            }
            ImageSource::Bytes(bytes) => {
                if let Ok(img) = image::load_from_memory(bytes) {
                    img.dimensions()
                } else {
                    (100, 100)
                }
            }
        }
    }

    fn measure_text(&self, text: &str, font_size: f32) -> (f32, f32) {
        let scaled_font = self.font.as_scaled(font_size);

        let mut width = 0.0f32;
        let mut prev_glyph: Option<ab_glyph::GlyphId> = None;

        for ch in text.chars() {
            let glyph_id = self.font.glyph_id(ch);

            if let Some(prev) = prev_glyph {
                width += scaled_font.kern(prev, glyph_id);
            }

            width += scaled_font.h_advance(glyph_id);
            prev_glyph = Some(glyph_id);
        }

        let height = scaled_font.height();

        (width, height)
    }

    fn extract_layout(
        &self,
        node_id: NodeId,
        offset_x: f32,
        offset_y: f32,
        result: &mut Vec<LayoutNode>,
    ) {
        let layout = self.taffy.layout(node_id).expect("Node should have layout");

        let rect = Rect {
            x: offset_x + layout.location.x,
            y: offset_y + layout.location.y,
            width: layout.size.width,
            height: layout.size.height,
        };

        let data = self.taffy.get_node_context(node_id).cloned();

        result.push(LayoutNode { rect, data });

        for child_id in self.taffy.children(node_id).expect("Should get children") {
            self.extract_layout(child_id, rect.x, rect.y, result);
        }
    }
}

impl Default for LayoutEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct LayoutTree {
    pub nodes: Vec<LayoutNode>,
}

#[derive(Debug, Clone)]
pub struct LayoutNode {
    pub rect: Rect,
    pub data: Option<NodeData>,
}

fn convert_direction(dir: ProtonDir) -> taffy::FlexDirection {
    match dir {
        ProtonDir::Row => taffy::FlexDirection::Row,
        ProtonDir::Column => taffy::FlexDirection::Column,
    }
}

fn convert_justify(j: ProtonJustify) -> JustifyContent {
    match j {
        ProtonJustify::Start => JustifyContent::Start,
        ProtonJustify::End => JustifyContent::End,
        ProtonJustify::Center => JustifyContent::Center,
        ProtonJustify::SpaceBetween => JustifyContent::SpaceBetween,
        ProtonJustify::SpaceAround => JustifyContent::SpaceAround,
        ProtonJustify::SpaceEvenly => JustifyContent::SpaceEvenly,
    }
}

fn convert_align(a: ProtonAlign) -> AlignItems {
    match a {
        ProtonAlign::Start => AlignItems::Start,
        ProtonAlign::End => AlignItems::End,
        ProtonAlign::Center => AlignItems::Center,
        ProtonAlign::Stretch => AlignItems::Stretch,
    }
}

fn convert_dimension(dim: ProtonDim) -> Dimension {
    match dim {
        ProtonDim::Auto => Dimension::Auto,
        ProtonDim::Px(px) => Dimension::Length(px),
        ProtonDim::Percent(p) => Dimension::Percent(p),
    }
}
