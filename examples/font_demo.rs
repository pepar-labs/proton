use anyhow::Result;
use it8951::Device;
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
fn main() -> Result<()> {
    let mut device = Device::connect()?;
    device.set_rotation(it8951::Rotation::Rotate270);
    let (width, height) = device.dimensions();
    device.clear_framebuffer();

    let ui = View::column()
        .padding(20.0)
        .gap(20.0)
        .background(Color::Gray(180))
        .width(Dimension::Percent(1.0))
        .height(Dimension::Percent(1.0))
        .align(Align::Center)
        .child(Text::new("Font Showcase").size(48.0).bold())
        .child(
            View::column()
                .background(Color::Gray(180))
                .gap(60.0)
                .justify(Justify::Center)
                .align(Align::Center)
                .child(Text::new("Regular: The quick brown fox jumps over the lazy dog").size(24.0))
                .child(
                    Text::new("Bold: The quick brown fox jumps over the lazy dog")
                        .size(100.0)
                        .align(TextAlign::Center)
                        .wrap(TextWrap::Word)
                        .bold(),
                )
                .child(
                    Text::new("Italic: The quick brown fox jumps over the lazy dog")
                        .size(44.0)
                        .wrap(TextWrap::Word)
                        .italic(),
                )
                .child(
                    Text::new("Bold Italic: The quick brown fox")
                        .size(44.0)
                        .wrap(TextWrap::Word)
                        .font(TextFont::NotosansBoldItalic),
                )
                .child(
                    Text::new("Mono: fn main() { println!(\"Hello!\"); }")
                        .size(44.0)
                        .wrap(TextWrap::Word)
                        .mono(),
                ),
        )
        .child(
            View::column()
                .padding(20.0)
                .gap(15.0)
                .background(Color::Gray(240))
                .child(Text::new("Mixed Styles Example").size(62.0).bold())
                .child(Text::new("This is regular text for body content.").size(60.0))
                .child(
                    Text::new("This is emphasized text using italic style.")
                        .size(60.0)
                        .italic(),
                )
                .child(Text::new("let code = \"monospace\";").size(58.0).mono()),
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
