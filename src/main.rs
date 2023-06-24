extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

struct Image {
    info: png::OutputInfo,
    bytes: Vec<u8>,
}

fn load_image_data(path: &str) -> Image {
    use std::fs::File;
    // The decoder is a build for reader and can be used to set various decoding options
    // via `Transformations`. The default output transformation is `Transformations::IDENTITY`.
    let decoder = png::Decoder::new(File::open(path).unwrap());
    let mut reader = decoder.read_info().unwrap();
    // Allocate the output buffer.
    let mut buf = vec![0; reader.output_buffer_size()];
    // Read the next frame. An APNG might contain multiple frames.
    let info = reader.next_frame(&mut buf).unwrap();
    println!("What is info? {info:?}");

    // Grab the bytes of the image.
    let bytes = &buf[..info.buffer_size()];
    // Inspect more details of the last read frame.
    // let in_animation = reader.info().frame_control.is_some();
    let bytes: Vec<u8> = bytes.to_vec();
    return Image { info, bytes };
}
pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let crab_image_data = load_image_data("textures/crabs.png");

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let mut crab_texture = texture_creator
        .create_texture_static(
            sdl2::pixels::PixelFormatEnum::ABGR8888,
            crab_image_data.info.width,
            crab_image_data.info.height,
        )
        .expect("Failed to create crab texture");
    crab_texture
        .update(None, &crab_image_data.bytes, crab_image_data.info.line_size)
        .expect("Failed to populate texture with crab");
    crab_texture.set_blend_mode(sdl2::render::BlendMode::Blend);

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(255, 210, 0));
        // A draw a rectangle which almost fills our window with it !
        canvas
            .fill_rect(Rect::new(10, 10, 780, 580))
            .expect("Could not fill rect");
        canvas
            .copy(&crab_texture, None, Rect::new(10, 10, 80, 580))
            .expect("Could not draw crab");
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
