extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;
use std::time::Duration;

struct Image {
    info: png::OutputInfo,
    bytes: Vec<u8>,
}

struct SpriteSheet<'a> {
    texture: Texture<'a>,
    image: Image,
    path: &'a str,
    segments_x: u32,
    segments_y: u32,
    segment_width: u32,
    segment_height: u32,
    total_sprites: u32,
}

impl<'a> SpriteSheet<'a> {
    pub fn new(
        path: &'a str,
        texture_creator: &'a TextureCreator<WindowContext>,
        segments_x: u32,
        segments_y: u32,
    ) -> SpriteSheet<'a> {
        let image = load_image_data("textures/crabs.png");
        let segment_width = image.info.width / segments_x;
        let segment_height = image.info.height / segments_y;
        let total_sprites = segments_x * segments_y;
        let mut texture: Texture<'a> = texture_creator
            .create_texture_static(
                sdl2::pixels::PixelFormatEnum::ABGR8888,
                image.info.width,
                image.info.height,
            )
            .expect("Failed to create crab texture");
        texture
            .update(None, &image.bytes, image.info.line_size)
            .expect("Failed to populate texture with crab");
        texture.set_blend_mode(sdl2::render::BlendMode::Blend);
        return SpriteSheet {
            texture,
            image,
            path,
            segments_x,
            segments_y,
            segment_width,
            segment_height,
            total_sprites,
        };
    }
    pub fn draw(&self, canvas: &mut WindowCanvas, sprite_index: u32, x: i32, y: i32) {
        let source_x: i32 = (self.segment_width as i32) * (sprite_index % self.segments_x) as i32;
        let source_y: i32 = (self.segment_height as i32) * (sprite_index / self.segments_x) as i32;
        let source_rect = Rect::new(source_x, source_y, self.segment_width, self.segment_height);
        let dest_rect = Rect::new(x, y, self.segment_width, self.segment_height);
        canvas
            .copy(&self.texture, source_rect, dest_rect)
            .expect("Could not draw crab");
    }
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

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Error creating canvas from window");
    let texture_creator = canvas.texture_creator();

    let crab_sprite_sheet = SpriteSheet::new("textures/crabs.png", &texture_creator, 1, 7);

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    let mut counter = 0;
    let mut sprite_frame = 0;
    'running: loop {
        i = (i + 1) % 255;
        let bg_color = Color::RGB(i, 64, 255 - i);
        canvas.set_draw_color(bg_color);
        canvas.clear();
        canvas.set_draw_color(Color::RGB(255, 210, 0));
        // A draw a rectangle which almost fills our window with it !
        canvas
            .fill_rect(Rect::new(10, 10, 780, 580))
            .expect("Could not fill rect");
        canvas.set_draw_color(bg_color);
        canvas
            .fill_rect(Rect::new(20, 20, 760, 560))
            .expect("Could not fill rect");
        crab_sprite_sheet.draw(
            &mut canvas,
            (0 + sprite_frame) % crab_sprite_sheet.total_sprites,
            10,
            10,
        );
        crab_sprite_sheet.draw(
            &mut canvas,
            (1 + sprite_frame) % crab_sprite_sheet.total_sprites,
            110,
            110,
        );
        crab_sprite_sheet.draw(
            &mut canvas,
            (2 + sprite_frame) % crab_sprite_sheet.total_sprites,
            512,
            256,
        );
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

        counter = (counter + 1) % 10;
        if counter == 9 {
            sprite_frame += 1;
        }
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
