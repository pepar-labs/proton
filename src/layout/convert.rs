use taffy::prelude::*;

use crate::style::{
    Align as ProtonAlign, Dimension as ProtonDim, FlexDirection as ProtonDir,
    Justify as ProtonJustify,
};

pub fn convert_direction(dir: ProtonDir) -> taffy::FlexDirection {
    match dir {
        ProtonDir::Row => taffy::FlexDirection::Row,
        ProtonDir::Column => taffy::FlexDirection::Column,
    }
}

pub fn convert_justify(j: ProtonJustify) -> JustifyContent {
    match j {
        ProtonJustify::Start => JustifyContent::Start,
        ProtonJustify::End => JustifyContent::End,
        ProtonJustify::Center => JustifyContent::Center,
        ProtonJustify::SpaceBetween => JustifyContent::SpaceBetween,
        ProtonJustify::SpaceAround => JustifyContent::SpaceAround,
        ProtonJustify::SpaceEvenly => JustifyContent::SpaceEvenly,
    }
}

pub fn convert_align(a: ProtonAlign) -> AlignItems {
    match a {
        ProtonAlign::Start => AlignItems::Start,
        ProtonAlign::End => AlignItems::End,
        ProtonAlign::Center => AlignItems::Center,
        ProtonAlign::Stretch => AlignItems::Stretch,
    }
}

pub fn convert_dimension(dim: ProtonDim) -> Dimension {
    match dim {
        ProtonDim::Auto => Dimension::Auto,
        ProtonDim::Px(px) => Dimension::Length(px),
        ProtonDim::Percent(p) => Dimension::Percent(p),
    }
}
