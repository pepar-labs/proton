use driver::{Device, Mode, Rotation};
use proton::prelude::*;

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

fn main() -> anyhow::Result<()> {
    println!("Connecting to e-ink display...");
    let mut device = Device::connect()?;

    let info = device.get_system_info().expect("Failed to get system info");
    println!("Display hardware: {}x{}", info.width, info.height);

    device.set_rotation(Rotation::Rotate270);
    let (width, height) = device.dimensions();
    println!("Portrait mode (logical): {}x{}", width, height);

    device.clear_framebuffer();

    let ui = View::column()
        .padding(20.0)
        .gap(20.0)
        .background(Color::White)
        .width(Dimension::Percent(1.0))
        .height(Dimension::Percent(1.0))
        .child(Text::new("Hello, Proton!").size(60.0).color(Color::Black))
        .child(
            View::row()
                .gap(15.0)
                .child(Text::new("E-Ink").size(32.0).color(Color::Black))
                .child(Text::new("UI").size(32.0).color(Color::Black))
                .child(Text::new("Framework").size(32.0).color(Color::Black)),
        )
        .child(
            Text::new("Powered by taffy + ab_glyph")
                .size(74.0)
                .color(Color::Black),
        )
        .child(
            View::column()
                .padding(20.0)
                .gap(10.0)
                .align(Align::Center)
                .justify(Justify::Center)
                .background(Color::Gray(230))
                .child(Text::new("Features:").size(28.0).color(Color::Black))
                .child(
                    Text::new("- Flexbox layout")
                        .size(22.0)
                        .color(Color::Gray(60)),
                )
                .child(
                    Text::new("- Declarative API")
                        .size(22.0)
                        .color(Color::Gray(60)),
                )
                .child(
                    Text::new("- Grayscale rendering")
                        .size(22.0)
                        .color(Color::Gray(60)),
                )
                .child(
                    Text::new("- Direct framebuffer rendering")
                        .size(22.0)
                        .color(Color::Gray(60)),
                ),
        )
        .child(
            View::row()
                .gap(20.0)
                .align(Align::Center)
                .justify(Justify::Center)
                .background(Color::Gray(200))
                .child(
                    Image::from_path("/home/calc/Pictures/cute-rac.png")
                        .height(Dimension::Px(500.0))
                        .width(Dimension::Px(500.0))
                        .fit(ImageFit::Contain),
                )
                .child(
                    Image::from_path("/home/calc/Pictures/baby-rac.jpg")
                        .height(Dimension::Px(500.0))
                        .width(Dimension::Px(500.0))
                        .fit(ImageFit::Contain),
                ),
        )
        .build();

    println!("Computing layout");
    let mut engine = LayoutEngine::new();
    let size = Size::new(width as f32, height as f32);
    let layout = engine.compute(&ui, size);

    println!("Rendering to framebuffer");
    let renderer = Renderer::new();
    {
        let mut target = DeviceTarget {
            device: &mut device,
            width,
            height,
        };
        renderer.render_to(&mut target, &layout, &ui);
    }

    println!("Flushing to display...");
    device.flush(Mode::GLD16)?;

    println!("Done");
    Ok(())
}
