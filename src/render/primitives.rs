use crate::style::{Color, Rect};

use super::target::RenderTarget;

pub fn is_within_clip(x: i32, y: i32, clip: Option<&Rect>) -> bool {
    match clip {
        Some(rect) => {
            let clip_x_start = rect.x as i32;
            let clip_y_start = rect.y as i32;
            let clip_x_end = (rect.x + rect.width) as i32;
            let clip_y_end = (rect.y + rect.height) as i32;
            x >= clip_x_start && x < clip_x_end && y >= clip_y_start && y < clip_y_end
        }
        None => true,
    }
}

pub fn fill_rect_clipped<T: RenderTarget>(
    target: &mut T,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    color: Color,
    clip: Option<&Rect>,
) {
    let luma = color.to_luma();
    let mut x_start = x.max(0.0) as i32;
    let mut y_start = y.max(0.0) as i32;
    let mut x_end = (x + width) as i32;
    let mut y_end = (y + height) as i32;

    if let Some(clip_rect) = clip {
        x_start = x_start.max(clip_rect.x as i32);
        y_start = y_start.max(clip_rect.y as i32);
        x_end = x_end.min((clip_rect.x + clip_rect.width) as i32);
        y_end = y_end.min((clip_rect.y + clip_rect.height) as i32);
    }

    for py in y_start..y_end {
        for px in x_start..x_end {
            target.set_pixel(px, py, luma);
        }
    }
}
