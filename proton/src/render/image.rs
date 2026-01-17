use crate::layout::LayoutNode;
use crate::nodes::{ImageNode, ImageSource};
use crate::style::{ImageFit, Rect};

use super::primitives::is_within_clip;
use super::target::RenderTarget;

pub fn render_image<T: RenderTarget>(
    target: &mut T,
    img: &ImageNode,
    layout_node: &LayoutNode,
    clip: Option<&Rect>,
) {
    let dynamic_img = match &img.source {
        ImageSource::Path(path) => match image::open(path) {
            Ok(img) => img,
            Err(_) => return,
        },
        ImageSource::Bytes(bytes) => match image::load_from_memory(bytes) {
            Ok(img) => img,
            Err(_) => return,
        },
    };

    let gray_img = dynamic_img.to_luma8();
    let (img_width, img_height) = gray_img.dimensions();

    let rect = &layout_node.rect;
    let container_width = rect.width;
    let container_height = rect.height;

    let (scale_x, scale_y, offset_x, offset_y, draw_width, draw_height) = calculate_image_fit(
        img_width as f32,
        img_height as f32,
        container_width,
        container_height,
        img.fit,
    );

    let dest_x_start = rect.x as i32 + offset_x as i32;
    let dest_y_start = rect.y as i32 + offset_y as i32;

    for dy in 0..(draw_height as i32) {
        for dx in 0..(draw_width as i32) {
            let dest_x = dest_x_start + dx;
            let dest_y = dest_y_start + dy;

            if is_within_clip(dest_x, dest_y, clip) {
                let src_x = (dx as f32 / scale_x) as u32;
                let src_y = (dy as f32 / scale_y) as u32;

                if src_x < img_width && src_y < img_height {
                    let pixel = gray_img.get_pixel(src_x, src_y).0[0];
                    target.set_pixel(dest_x, dest_y, pixel);
                }
            }
        }
    }
}

fn calculate_image_fit(
    img_width: f32,
    img_height: f32,
    container_width: f32,
    container_height: f32,
    fit: ImageFit,
) -> (f32, f32, f32, f32, f32, f32) {
    match fit {
        ImageFit::Fill => {
            let scale_x = container_width / img_width;
            let scale_y = container_height / img_height;
            (
                scale_x,
                scale_y,
                0.0,
                0.0,
                container_width,
                container_height,
            )
        }
        ImageFit::Contain => {
            let scale = (container_width / img_width).min(container_height / img_height);
            let draw_width = img_width * scale;
            let draw_height = img_height * scale;
            let offset_x = (container_width - draw_width) / 2.0;
            let offset_y = (container_height - draw_height) / 2.0;
            (scale, scale, offset_x, offset_y, draw_width, draw_height)
        }
        ImageFit::Cover => {
            let scale = (container_width / img_width).max(container_height / img_height);
            let draw_width = container_width;
            let draw_height = container_height;
            let offset_x = 0.0;
            let offset_y = 0.0;
            (scale, scale, offset_x, offset_y, draw_width, draw_height)
        }
        ImageFit::None => {
            let draw_width = img_width.min(container_width);
            let draw_height = img_height.min(container_height);
            let offset_x = (container_width - draw_width) / 2.0;
            let offset_y = (container_height - draw_height) / 2.0;
            (1.0, 1.0, offset_x, offset_y, draw_width, draw_height)
        }
    }
}
