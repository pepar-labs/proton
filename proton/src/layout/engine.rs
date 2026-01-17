use image::GenericImageView;
use taffy::prelude::*;

use crate::font::Fonts;
use crate::nodes::{ImageNode, ImageSource, Node, ScrollViewNode, TextNode, ViewNode};
use crate::style::{Dimension as ProtonDim, Rect, Size, TextWrap};
use crate::text::{line_height, measure_text_width, wrap_text};

use super::convert::{convert_align, convert_dimension, convert_direction, convert_justify};
use super::node_data::NodeData;
use super::tree::{LayoutNode, LayoutTree};

pub struct LayoutEngine {
    taffy: TaffyTree<NodeData>,
    fonts: Fonts,
}

impl LayoutEngine {
    pub fn new() -> Self {
        Self {
            taffy: TaffyTree::new(),
            fonts: Fonts::new(),
        }
    }

    pub fn compute(&mut self, root: &Node, available: Size) -> LayoutTree {
        self.taffy.clear();

        let root_id = self.build_taffy_node(root);

        let fonts = self.fonts.clone();

        self.taffy
            .compute_layout_with_measure(
                root_id,
                taffy::Size {
                    width: AvailableSpace::Definite(available.width),
                    height: AvailableSpace::Definite(available.height),
                },
                |known_dimensions, available_space, _node_id, node_context, _style| {
                    measure_node(&fonts, known_dimensions, available_space, node_context)
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
            Node::ScrollView(scroll) => self.build_scroll_view_node(scroll),
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
        let style = Style {
            ..Default::default()
        };

        self.taffy
            .new_leaf_with_context(
                style,
                NodeData::Text {
                    content: text.content.clone(),
                    font_size: text.font_size,
                    wrap: text.wrap,
                    overflow: text.overflow,
                    align: text.align,
                    font: text.font,
                },
            )
            .expect("Failed to create text node")
    }

    fn build_image_node(&mut self, img: &ImageNode) -> NodeId {
        let (intrinsic_width, intrinsic_height) = get_image_dimensions(&img.source);

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

    fn build_scroll_view_node(&mut self, scroll: &ScrollViewNode) -> NodeId {
        let child_ids: Vec<NodeId> = scroll
            .children
            .iter()
            .map(|child| self.build_taffy_node(child))
            .collect();

        let style = Style {
            display: Display::Flex,
            flex_direction: convert_direction(scroll.direction),
            justify_content: Some(convert_justify(scroll.justify)),
            align_items: Some(convert_align(scroll.align)),
            padding: taffy::Rect {
                left: LengthPercentage::Length(scroll.padding),
                right: LengthPercentage::Length(scroll.padding),
                top: LengthPercentage::Length(scroll.padding),
                bottom: LengthPercentage::Length(scroll.padding),
            },
            gap: taffy::Size {
                width: LengthPercentage::Length(scroll.gap),
                height: LengthPercentage::Length(scroll.gap),
            },
            size: taffy::Size {
                width: convert_dimension(scroll.width),
                height: convert_dimension(scroll.height),
            },
            // Allow content to overflow - this is what makes it scrollable
            overflow: taffy::Point {
                x: taffy::Overflow::Visible,
                y: taffy::Overflow::Scroll,
            },
            ..Default::default()
        };

        self.taffy
            .new_with_children(style, &child_ids)
            .map(|node_id| {
                // We'll compute content_height after layout, for now use 0
                self.taffy
                    .set_node_context(
                        node_id,
                        Some(NodeData::ScrollView {
                            scroll_offset: scroll.scroll_offset,
                            content_height: 0.0,
                        }),
                    )
                    .expect("Failed to set scroll view context");
                node_id
            })
            .expect("Failed to create scroll view node")
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

        let mut data = self.taffy.get_node_context(node_id).cloned();

        // For ScrollView, calculate the actual content height from children
        if let Some(NodeData::ScrollView {
            scroll_offset,
            content_height: _,
        }) = &data
        {
            let children = self.taffy.children(node_id).expect("Should get children");
            let mut total_content_height = 0.0f32;

            for child_id in &children {
                let child_layout = self
                    .taffy
                    .layout(*child_id)
                    .expect("Child should have layout");
                let child_bottom = child_layout.location.y + child_layout.size.height;
                total_content_height = total_content_height.max(child_bottom);
            }

            data = Some(NodeData::ScrollView {
                scroll_offset: *scroll_offset,
                content_height: total_content_height,
            });
        }

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

fn measure_node(
    fonts: &Fonts,
    known_dimensions: taffy::Size<Option<f32>>,
    available_space: taffy::Size<AvailableSpace>,
    node_context: Option<&mut NodeData>,
) -> taffy::Size<f32> {
    if let taffy::Size {
        width: Some(w),
        height: Some(h),
    } = known_dimensions
    {
        return taffy::Size {
            width: w,
            height: h,
        };
    }

    match node_context {
        Some(NodeData::Text {
            content,
            font_size,
            wrap,
            font: text_font,
            ..
        }) => measure_text_wrapped(
            fonts.get(*text_font),
            content,
            *font_size,
            *wrap,
            known_dimensions,
            available_space,
        ),
        Some(NodeData::Image { .. }) => taffy::Size::ZERO,
        _ => taffy::Size::ZERO,
    }
}

fn measure_text_wrapped(
    font: &ab_glyph::FontRef<'static>,
    text: &str,
    font_size: f32,
    wrap: TextWrap,
    known_dimensions: taffy::Size<Option<f32>>,
    available_space: taffy::Size<AvailableSpace>,
) -> taffy::Size<f32> {
    let font_line_height = line_height(font, font_size);

    if wrap == TextWrap::None {
        let width = measure_text_width(font, text, font_size);
        return taffy::Size {
            width: known_dimensions.width.unwrap_or(width),
            height: known_dimensions.height.unwrap_or(font_line_height),
        };
    }

    let available_width = known_dimensions
        .width
        .unwrap_or(match available_space.width {
            AvailableSpace::Definite(w) => w,
            AvailableSpace::MaxContent => f32::MAX,
            AvailableSpace::MinContent => 0.0,
        });

    let lines = wrap_text(font, text, font_size, available_width, wrap);

    let max_line_width = lines
        .iter()
        .map(|line| measure_text_width(font, line, font_size))
        .fold(0.0f32, f32::max);

    let total_height = lines.len() as f32 * font_line_height;

    taffy::Size {
        width: known_dimensions.width.unwrap_or(max_line_width),
        height: known_dimensions.height.unwrap_or(total_height),
    }
}

fn get_image_dimensions(source: &ImageSource) -> (u32, u32) {
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
