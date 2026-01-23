use anyhow::{Ok, Result};
use it8951::Device;
use proton::{
    nodes::{Image, View},
    Dimension, LayoutEngine, RenderTarget, Renderer, Size,
};

struct DeviceTarget<'a> {
    device: &'a mut Device,
    width: u32,
    height: u32,
}

impl<'a> RenderTarget for DeviceTarget<'a> {
    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn set_pixel(&mut self, x: i32, y: i32, color: u8) {
        self.device.set_pixel(x, y, color);
    }

    fn get_pixel(&self, x: i32, y: i32) -> u8 {
        self.device.get_pixel(x, y)
    }
}

fn main() -> Result<()> {
    let mut device = Device::connect()?;
    device.set_rotation(it8951::Rotation::Rotate270);
    let (width, height) = device.dimensions();
    device.clear_framebuffer();

    let ui = View::column()
        .width(Dimension::Percent(1.0))
        .height(Dimension::Percent(1.0))
        .justify(proton::Justify::Center)
        .align(proton::Align::Center)
        // .child(
        //     View::row()
        //         .background(proton::Color::Black)
        //         .child(
        //             Text::new("Jujutsu Kaisen")
        //                 .size(70.0)
        //                 .color(proton::Color::White),
        //         )
        //         .width(Dimension::Percent(1.0))
        //         .justify(proton::Justify::Center)
        //         .align(Align::Center),
        // )
        .child(
            Image::from_path("/home/calc/Pictures/chizuru.webp")
                .height(Dimension::Percent(1.0))
                .width(Dimension::Percent(1.0))
                .fit(proton::ImageFit::Fill),
        )
        .build();

    let mut engine = LayoutEngine::new();
    let size = Size::new(width as f32, height as f32);
    let layout = engine.compute(&ui, size);

    let renderer = Renderer::new();
    {
        let mut target = DeviceTarget {
            device: &mut device,
            width,
            height,
        };
        renderer.render_to(&mut target, &layout, &ui);
    }

    device.flush(it8951::Mode::GLD16)?;
    Ok(())
}
