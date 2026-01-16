use crate::nodes::{ImageNode, ImageSource, Node, TextNode, ViewNode};
use crate::style::{
    Align as ProtonAlign, Dimension as ProtonDim, FlexDirection as ProtonDir, ImageFit,
    Justify as ProtonJustify, Rect, Size, TextAlign, TextOverflow, TextWrap,
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
    Text {
        content: String,
        font_size: f32,
        wrap: TextWrap,
        overflow: TextOverflow,
        align: TextAlign,
    },
    Image {
        source: ImageSource,
        fit: ImageFit,
    },
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

        let font = self.font.clone();

        self.taffy
            .compute_layout_with_measure(
                root_id,
                taffy::Size {
                    width: AvailableSpace::Definite(available.width),
                    height: AvailableSpace::Definite(available.height),
                },
                |known_dimensions, available_space, _node_id, node_context, _style| {
                    measure_node(&font, known_dimensions, available_space, node_context)
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

fn measure_node(
    font: &ab_glyph::FontRef<'static>,
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
            ..
        }) => measure_text_wrapped(
            font,
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
    let scaled_font = font.as_scaled(font_size);
    let line_height = scaled_font.height();

    if wrap == TextWrap::None {
        let width = measure_text_width(font, text, font_size);
        return taffy::Size {
            width: known_dimensions.width.unwrap_or(width),
            height: known_dimensions.height.unwrap_or(line_height),
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

    let total_height = lines.len() as f32 * line_height;

    taffy::Size {
        width: known_dimensions.width.unwrap_or(max_line_width),
        height: known_dimensions.height.unwrap_or(total_height),
    }
}

fn measure_text_width(font: &ab_glyph::FontRef<'static>, text: &str, font_size: f32) -> f32 {
    let scaled_font = font.as_scaled(font_size);
    let mut width = 0.0f32;
    let mut prev_glyph: Option<ab_glyph::GlyphId> = None;

    for ch in text.chars() {
        let glyph_id = font.glyph_id(ch);

        if let Some(prev) = prev_glyph {
            width += scaled_font.kern(prev, glyph_id);
        }

        width += scaled_font.h_advance(glyph_id);
        prev_glyph = Some(glyph_id);
    }

    width
}

fn measure_char_width(font: &ab_glyph::FontRef<'static>, ch: char, font_size: f32) -> f32 {
    let scaled_font = font.as_scaled(font_size);
    let glyph_id = font.glyph_id(ch);
    scaled_font.h_advance(glyph_id)
}

pub fn wrap_text(
    font: &ab_glyph::FontRef<'static>,
    text: &str,
    font_size: f32,
    max_width: f32,
    wrap: TextWrap,
) -> Vec<String> {
    match wrap {
        TextWrap::None => vec![text.to_string()],
        TextWrap::Word => wrap_words(font, text, font_size, max_width, false),
        TextWrap::Char => wrap_chars(font, text, font_size, max_width),
        TextWrap::WordChar => wrap_words(font, text, font_size, max_width, true),
    }
}

fn wrap_words(
    font: &ab_glyph::FontRef<'static>,
    text: &str,
    font_size: f32,
    max_width: f32,
    break_long: bool,
) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();
    let mut current_width = 0.0f32;
    let space_width = measure_char_width(font, ' ', font_size);

    for word in text.split_whitespace() {
        let word_width = measure_text_width(font, word, font_size);

        let needed_width = if current_line.is_empty() {
            word_width
        } else {
            space_width + word_width
        };

        if current_width + needed_width <= max_width {
            if !current_line.is_empty() {
                current_line.push(' ');
                current_width += space_width;
            }
            current_line.push_str(word);
            current_width += word_width;
        } else if word_width > max_width && break_long {
            if !current_line.is_empty() {
                lines.push(current_line);
                current_line = String::new();
                current_width = 0.0;
            }
            let char_lines = wrap_chars(font, word, font_size, max_width);
            let char_lines_len = char_lines.len();
            for (i, char_line) in char_lines.into_iter().enumerate() {
                if i == char_lines_len - 1 {
                    current_line = char_line;
                    current_width = measure_text_width(font, &current_line, font_size);
                } else {
                    lines.push(char_line);
                }
            }
        } else {
            if !current_line.is_empty() {
                lines.push(current_line);
            }
            current_line = word.to_string();
            current_width = word_width;
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    if lines.is_empty() {
        lines.push(String::new());
    }

    lines
}

fn wrap_chars(
    font: &ab_glyph::FontRef<'static>,
    text: &str,
    font_size: f32,
    max_width: f32,
) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();
    let mut current_width = 0.0f32;

    for ch in text.chars() {
        let char_width = measure_char_width(font, ch, font_size);

        if current_width + char_width > max_width && !current_line.is_empty() {
            lines.push(current_line);
            current_line = String::new();
            current_width = 0.0;
        }

        current_line.push(ch);
        current_width += char_width;
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    if lines.is_empty() {
        lines.push(String::new());
    }

    lines
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
