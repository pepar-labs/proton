use crate::nodes::ImageSource;
use crate::style::{ImageFit, TextAlign, TextFont, TextOverflow, TextWrap};

#[derive(Debug, Clone)]
pub enum NodeData {
    View,
    Text {
        content: String,
        font_size: f32,
        wrap: TextWrap,
        overflow: TextOverflow,
        align: TextAlign,
        font: TextFont,
    },
    Image {
        source: ImageSource,
        fit: ImageFit,
    },
    ScrollView {
        scroll_offset: f32,
        content_height: f32,
    },
}
