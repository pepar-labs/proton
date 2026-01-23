use image::{GrayImage, Luma};

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
