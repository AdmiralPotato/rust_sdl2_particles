use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;

pub struct Image {
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

pub struct SpriteSheet<'a> {
    pub texture: Texture<'a>,
    pub image: Image,
    pub path: &'a str,
    pub segments_x: u32,
    pub segments_y: u32,
    pub segment_width: u32,
    pub segment_height: u32,
    pub total_sprites: u32,
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
