use image::{GrayImage, Luma};

pub trait DeviceAPI {
    fn set_pixel(&mut self, x: i32, y: i32, color: u8);
    fn get_pixel(&self, x: i32, y: i32) -> u8;
    fn dimensions(&self) -> (u32, u32);
    fn flush(&mut self, mode: DisplayMode) -> Result<(), anyhow::Error>;
    fn clear_framebuffer(&mut self);
    fn set_rotation(&mut self, rotation: Rotation);
    fn rotation(&self) -> Rotation;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DisplayMode {
    CLEAR = 0,
    DU,
    GC16,
    GL16,
    GLR16,
    GLD16,
    DU4,
    A2,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Rotation {
    Rotate0,
    Rotate90,
    Rotate180,
    Rotate270,
}

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
