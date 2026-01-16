use ab_glyph::{point, Font, FontRef, ScaleFont};
use image::{GrayImage, Luma};

use crate::layout::{LayoutNode, LayoutTree};
use crate::nodes::Node;
use crate::nodes::{ImageNode, ImageSource, ViewNode};
use crate::style::{Color, ImageFit, Size};

pub trait RenderTarget {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn set_pixel(&mut self, x: i32, y: i32, color: u8);
    fn get_pixel(&self, x: i32, y: i32) -> u8;
}

impl RenderTarget for GrayImage {
    fn width(&self) -> u32 {
        image::GenericImageView::width(self)
    }

    fn height(&self) -> u32 {
        image::GenericImageView::height(self)
    }

    fn set_pixel(&mut self, x: i32, y: i32, color: u8) {
        if x >= 0 && y >= 0 && (x as u32) < self.width() && (y as u32) < self.height() {
            image::GenericImage::put_pixel(self, x as u32, y as u32, Luma([color]));
        }
    }

    fn get_pixel(&self, x: i32, y: i32) -> u8 {
        if x >= 0 && y >= 0 && (x as u32) < self.width() && (y as u32) < self.height() {
            image::GenericImageView::get_pixel(self, x as u32, y as u32).0[0]
        } else {
            255
        }
    }
}

pub struct Renderer {
    font: FontRef<'static>,
}

impl Renderer {
    pub fn new() -> Self {
        let font_data: &'static [u8] = include_bytes!("../fonts/NotoSans-Regular.ttf");
        let font = FontRef::try_from_slice(font_data).expect("Failed to load embedded font");

        Self { font }
    }

    pub fn render(&self, layout: &LayoutTree, root: &Node, size: Size) -> GrayImage {
        let mut image = GrayImage::from_pixel(
            size.width as u32,
            size.height as u32,
            Luma([255u8]), // White background
        );
        self.render_to(&mut image, layout, root);
        image
    }

    pub fn render_to<T: RenderTarget>(&self, target: &mut T, layout: &LayoutTree, root: &Node) {
        self.render_node(target, root, layout, 0);
    }

    fn render_node<T: RenderTarget>(
        &self,
        target: &mut T,
        node: &Node,
        layout: &LayoutTree,
        index: usize,
    ) -> usize {
        let layout_node = &layout.nodes[index];

        match node {
            Node::View(view) => self.render_view(target, view, layout_node, layout, index),
            Node::Text(text) => {
                self.render_text(
                    target,
                    &text.content,
                    text.font_size,
                    text.color,
                    layout_node,
                );
                index + 1
            }
            Node::Image(img) => {
                self.render_image(target, img, layout_node);
                index + 1
            }
        }
    }

    fn render_view<T: RenderTarget>(
        &self,
        target: &mut T,
        view: &ViewNode,
        layout_node: &LayoutNode,
        layout: &LayoutTree,
        index: usize,
    ) -> usize {
        let rect = &layout_node.rect;

        if let Some(color) = view.background {
            self.fill_rect(target, rect.x, rect.y, rect.width, rect.height, color);
        }

        let mut next_index = index + 1;
        for child in &view.children {
            next_index = self.render_node(target, child, layout, next_index);
        }

        next_index
    }

    fn render_text<T: RenderTarget>(
        &self,
        target: &mut T,
        text: &str,
        font_size: f32,
        color: Color,
        layout_node: &LayoutNode,
    ) {
        let rect = &layout_node.rect;
        let scaled_font = self.font.as_scaled(font_size);

        let ascent = scaled_font.ascent();
        let baseline_y = rect.y + ascent;

        let mut cursor_x = rect.x;
        let mut prev_glyph: Option<ab_glyph::GlyphId> = None;

        let luma = color.to_luma();

        for ch in text.chars() {
            let glyph_id = self.font.glyph_id(ch);

            if let Some(prev) = prev_glyph {
                cursor_x += scaled_font.kern(prev, glyph_id);
            }

            let glyph = glyph_id.with_scale_and_position(font_size, point(cursor_x, baseline_y));

            if let Some(outlined) = self.font.outline_glyph(glyph) {
                let bounds = outlined.px_bounds();
                outlined.draw(|px, py, coverage| {
                    let x = bounds.min.x as i32 + px as i32;
                    let y = bounds.min.y as i32 + py as i32;

                    let existing = target.get_pixel(x, y);
                    let alpha = coverage;
                    let blended = ((1.0 - alpha) * existing as f32 + alpha * luma as f32) as u8;
                    target.set_pixel(x, y, blended);
                });
            }

            cursor_x += scaled_font.h_advance(glyph_id);
            prev_glyph = Some(glyph_id);
        }
    }

    fn fill_rect<T: RenderTarget>(
        &self,
        target: &mut T,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color: Color,
    ) {
        let luma = color.to_luma();
        let x_start = x.max(0.0) as i32;
        let y_start = y.max(0.0) as i32;
        let x_end = (x + width) as i32;
        let y_end = (y + height) as i32;

        for py in y_start..y_end {
            for px in x_start..x_end {
                target.set_pixel(px, py, luma);
            }
        }
    }

    fn render_image<T: RenderTarget>(
        &self,
        target: &mut T,
        img: &ImageNode,
        layout_node: &LayoutNode,
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

        let (scale_x, scale_y, offset_x, offset_y, draw_width, draw_height) = self
            .calculate_image_fit(
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

                let src_x = (dx as f32 / scale_x) as u32;
                let src_y = (dy as f32 / scale_y) as u32;

                if src_x < img_width && src_y < img_height {
                    let pixel = gray_img.get_pixel(src_x, src_y).0[0];
                    target.set_pixel(dest_x, dest_y, pixel);
                }
            }
        }
    }

    fn calculate_image_fit(
        &self,
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
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}
