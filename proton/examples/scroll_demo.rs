use driver::{Device, Mode, Rotation};
use proton::prelude::*;
use std::time::Duration;

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

fn build_ui(scroll_offset: f32) -> Node {
    View::column()
        .padding(20.0)
        .gap(20.0)
        .background(Color::White)
        .width(Dimension::Percent(1.0))
        .height(Dimension::Percent(1.0))
        .child(Text::new("ScrollView Demo").size(48.0).bold())
        .child(Text::new(format!("scroll_offset = {:.0}", scroll_offset)).size(24.0))
        .child(
            ScrollView::vertical()
                .width(Dimension::Percent(1.0))
                .height(Dimension::Px(900.0))
                .background(Color::Gray(240))
                .padding(20.0)
                .gap(20.0)
                .scroll_offset(scroll_offset)
                .child(Text::new("Item 1: This is the first item in the scroll view").size(32.0))
                .child(Text::new("Item 2: Second item with some text").size(32.0))
                .child(Text::new("Item 3: Third item here").size(32.0))
                .child(Text::new("Item 4: Fourth item content").size(32.0))
                .child(Text::new("Item 5: Fifth item in the list").size(32.0))
                .child(Text::new("Item 6: Sixth item below").size(32.0))
                .child(Text::new("Item 7: Seventh item").size(32.0))
                .child(Text::new("Item 8: Eighth item").size(32.0))
                .child(Text::new("Item 9: Ninth item").size(32.0))
                .child(Text::new("Item 10: Tenth item").size(32.0))
                .child(Text::new("Item 11: Eleventh item").size(32.0))
                .child(Text::new("Item 12: Twelfth item").size(32.0))
                .child(Text::new("Item 13: Thirteenth item").size(32.0))
                .child(Text::new("Item 14: Fourteenth item").size(32.0))
                .child(Text::new("Item 15: Fifteenth item").size(32.0))
                .child(Text::new("Item 16: Sixteenth item").size(32.0))
                .child(Text::new("Item 17: Seventeenth item").size(32.0))
                .child(Text::new("Item 18: Eighteenth item").size(32.0))
                .child(Text::new("Item 19: Nineteenth item").size(32.0))
                .child(Text::new("Item 20: End of content").size(32.0)),
        )
        .child(
            Text::new("Content below the ScrollView")
                .size(24.0)
                .italic(),
        )
        .build()
}

fn main() -> anyhow::Result<()> {
    println!("Connecting to e-ink display...");
    let mut device = Device::connect()?;

    let info = device.get_system_info().expect("Failed to get system info");
    println!("Display hardware: {}x{}", info.width, info.height);

    device.set_rotation(Rotation::Rotate270);
    let (width, height) = device.dimensions();
    println!("Portrait mode (logical): {}x{}", width, height);

    let mut engine = LayoutEngine::new();
    let renderer = Renderer::new();
    let size = Size::new(width as f32, height as f32);

    // Scroll down then back up
    let scroll_offsets = [
        0.0, 100.0, 200.0, 300.0, 400.0, 500.0, 400.0, 300.0, 200.0, 100.0, 0.0,
    ];

    for (i, &offset) in scroll_offsets.iter().enumerate() {
        println!("Frame {}: scroll_offset = {}", i + 1, offset);

        device.clear_framebuffer();

        let ui = build_ui(offset);
        let layout = engine.compute(&ui, size);

        {
            let mut target = DeviceTarget {
                device: &mut device,
                width,
                height,
            };
            renderer.render_to(&mut target, &layout, &ui);
        }

        // Use DU mode for fast monochrome updates
        device.flush(Mode::GLD16)?;

        // Small delay between frames
        std::thread::sleep(Duration::from_millis(300));
    }

    // Final render with high quality mode
    println!("Final high-quality render...");
    device.clear_framebuffer();
    let ui = build_ui(0.0);
    let layout = engine.compute(&ui, size);
    {
        let mut target = DeviceTarget {
            device: &mut device,
            width,
            height,
        };
        renderer.render_to(&mut target, &layout, &ui);
    }
    device.flush(Mode::GC16)?;

    println!("Done!");
    Ok(())
}
