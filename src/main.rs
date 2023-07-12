extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

mod spritesheet;
use spritesheet::SpriteSheet;
mod particle;
use particle::Particle;

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

    let mut crabicles: Vec<Particle> = vec![];
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    let mut counter = 0;
    let mut position: (i32, i32) = (0, 0);
    let mut last_crab_index: u32 = 0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::MouseButtonDown { x, y, .. } => {
                    println!("Event::MouseButtonDown? x: {x}, y: {y}");
                    crabicles.push(Particle::new(x as f32, y as f32, last_crab_index));
                    last_crab_index = last_crab_index.wrapping_add(1);
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
        crab_sprite_sheet.draw(&mut canvas, 0, 10, 10);
        crab_sprite_sheet.draw(&mut canvas, 1, 110, 110);
        // The rest of the game loop goes here...

        // for crab in crabicles {
        // equivalent to:
        // for crab in crabicles.into_iter() {
        // So if you plan to keep a vec around after looping over it,
        // .iter() or .iter_mut() for or it WILL BE CONSUMED~~!!!
        for (index, crab) in crabicles.iter_mut().enumerate() {
            crab.tick();
            crab_sprite_sheet.draw(
                &mut canvas,
                crab.index % crab_sprite_sheet.total_sprites,
                (crab.pos.x as i32) - 64,
                (crab.pos.y as i32) - 64,
            );
        }
        // this is like list = list.filter(...), but rustier
        crabicles.retain(|x| x.lifespan > 0);

        // crab for cursor
        crab_sprite_sheet.draw(&mut canvas, 2, position.0 - 64, position.1 - 64);

        counter = (counter + 1) % 10;
        if counter == 9 {}
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
