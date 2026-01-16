use driver::{Device, Rotation};
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
    // let output_width = 800u32;
    // let output_height = 600u32;

    let mut device = Device::connect()?;

    let info = device.get_system_info().expect("Failed to get system info");
    println!("Display hardware: {}x{}", info.width, info.height);

    device.set_rotation(Rotation::Rotate270);
    let (width, height) = device.dimensions();
    println!("Portrait mode (logical): {}x{}", width, height);

    device.clear_framebuffer();

    let long_text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris.";

    let ui = View::column()
        .padding(20.0)
        .gap(20.0)
        .background(Color::White)
        .width(Dimension::Percent(1.0))
        .height(Dimension::Percent(1.0))
        .child(Text::new("Text Wrap Demo").size(36.0).color(Color::Black))
        .child(
            View::row()
                .gap(20.0)
                .child(
                    View::column()
                        .gap(10.0)
                        .width(Dimension::Px(220.0))
                        .child(Text::new("No Wrap").size(16.0).color(Color::Gray(100)))
                        .child(
                            View::column()
                                .background(Color::Gray(240))
                                .padding(10.0)
                                .width(Dimension::Px(200.0))
                                .height(Dimension::Px(150.0))
                                .child(
                                    Text::new(long_text)
                                        .size(14.0)
                                        .color(Color::Black)
                                        .wrap(TextWrap::None),
                                ),
                        ),
                )
                .child(
                    View::column()
                        .gap(10.0)
                        .width(Dimension::Px(220.0))
                        .child(Text::new("Word Wrap").size(16.0).color(Color::Gray(100)))
                        .child(
                            View::column()
                                .background(Color::Gray(240))
                                .padding(10.0)
                                .width(Dimension::Px(200.0))
                                .height(Dimension::Px(150.0))
                                .child(
                                    Text::new(long_text)
                                        .size(14.0)
                                        .color(Color::Black)
                                        .wrap(TextWrap::Word),
                                ),
                        ),
                )
                .child(
                    View::column()
                        .gap(10.0)
                        .width(Dimension::Px(220.0))
                        .child(Text::new("Char Wrap").size(16.0).color(Color::Gray(100)))
                        .child(
                            View::column()
                                .background(Color::Gray(240))
                                .padding(10.0)
                                .width(Dimension::Px(200.0))
                                .height(Dimension::Px(150.0))
                                .child(
                                    Text::new(long_text)
                                        .size(14.0)
                                        .color(Color::Black)
                                        .wrap(TextWrap::Char),
                                ),
                        ),
                ),
        )
        .child(
            View::row()
                .gap(20.0)
                .child(
                    View::column()
                        .gap(10.0)
                        .width(Dimension::Px(220.0))
                        .child(Text::new("Left Align").size(16.0).color(Color::Gray(100)))
                        .child(
                            View::column()
                                .background(Color::Gray(240))
                                .padding(10.0)
                                .width(Dimension::Px(200.0))
                                .child(
                                    Text::new("Short text here")
                                        .size(14.0)
                                        .color(Color::Black)
                                        .wrap(TextWrap::Word)
                                        .align(TextAlign::Left),
                                ),
                        ),
                )
                .child(
                    View::column()
                        .gap(10.0)
                        .width(Dimension::Px(220.0))
                        .child(Text::new("Center Align").size(16.0).color(Color::Gray(100)))
                        .child(
                            View::column()
                                .background(Color::Gray(240))
                                .padding(10.0)
                                .width(Dimension::Px(200.0))
                                .child(
                                    Text::new("Short text here")
                                        .size(14.0)
                                        .color(Color::Black)
                                        .wrap(TextWrap::Word)
                                        .align(TextAlign::Center),
                                ),
                        ),
                )
                .child(
                    View::column()
                        .gap(10.0)
                        .width(Dimension::Px(220.0))
                        .child(Text::new("Right Align").size(16.0).color(Color::Gray(100)))
                        .child(
                            View::column()
                                .background(Color::Gray(240))
                                .padding(10.0)
                                .width(Dimension::Px(200.0))
                                .child(
                                    Text::new("Short text here")
                                        .size(14.0)
                                        .color(Color::Black)
                                        .wrap(TextWrap::Word)
                                        .align(TextAlign::Right),
                                ),
                        ),
                ),
        )
        .child(
            View::row()
                .gap(20.0)
                .child(
                    View::column()
                        .gap(10.0)
                        .width(Dimension::Px(220.0))
                        .child(
                            Text::new("Clip Overflow")
                                .size(16.0)
                                .color(Color::Gray(100)),
                        )
                        .child(
                            View::column()
                                .background(Color::Gray(240))
                                .padding(10.0)
                                .width(Dimension::Px(200.0))
                                .height(Dimension::Px(60.0))
                                .child(
                                    Text::new(format!("{}{}", long_text, long_text))
                                        .size(14.0)
                                        .color(Color::Black)
                                        .wrap(TextWrap::Word)
                                        .overflow(TextOverflow::Clip),
                                ),
                        ),
                )
                .child(
                    View::column()
                        .gap(10.0)
                        .width(Dimension::Px(220.0))
                        .height(Dimension::Px(222.0))
                        .child(
                            Text::new("Ellipsis Overflow")
                                .size(16.0)
                                .color(Color::Gray(100)),
                        )
                        .child(
                            View::column()
                                .background(Color::Gray(240))
                                .padding(10.0)
                                .width(Dimension::Px(200.0))
                                .height(Dimension::Px(60.0))
                                .child(
                                    Text::new(format!("{}{}{}", long_text, long_text, long_text))
                                        .size(14.0)
                                        .color(Color::Black)
                                        .wrap(TextWrap::Word)
                                        .overflow(TextOverflow::Clip),
                                ),
                        ),
                ),
        )
        .build();

    println!("Computing layout...");
    let mut engine = LayoutEngine::new();
    let size = Size::new(width as f32, height as f32);
    let layout = engine.compute(&ui, size);

    println!("Rendering...");
    let renderer = Renderer::new();
    {
        let mut target = DeviceTarget {
            device: &mut device,
            width,
            height,
        };
        renderer.render_to(&mut target, &layout, &ui);
    }
    device.flush(driver::Mode::GLD16)?;

    let output_path = "text_wrap_demo.png";
    println!("Saved to: {}", output_path);

    Ok(())
}
