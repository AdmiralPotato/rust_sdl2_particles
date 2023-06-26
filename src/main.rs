mod spritesheet;
use spritesheet::SpriteSheet;

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

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
    let mut position: (i32, i32) = (0, 0);
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::MouseButtonDown { x, y, .. } => {
                    println!("Event::MouseButtonDown? x: {x}, y: {y}")
                }
                Event::MouseMotion { x, y, .. } => {
                    // println!("Event::MouseMotion? x: {x}, y: {y}");
                    position.0 = x;
                    position.1 = y;
                }
                _ => {}
            }
        }
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
            position.0 - 64,
            position.1 - 64,
        );
        // The rest of the game loop goes here...

        counter = (counter + 1) % 10;
        if counter == 9 {
            sprite_frame += 1;
        }
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
