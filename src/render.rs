// use graphics::{color, rectangle::square, Rectangle};

use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::ViewPlane;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

pub fn run(view_plane: &ViewPlane) {
    let scale = 10;
    let width = view_plane.pixels.len() * scale;
    let sdl_context = sdl2::init().expect("blah");
    let video_subsystem = sdl_context.video().expect("blah");

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", width as u32, width as u32)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())
        .expect("blah");

    let mut canvas = window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())
        .expect("blah");

    let mut event_pump = sdl_context.event_pump().expect("blah");
    'running: loop {
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

        canvas.set_draw_color(Color::WHITE);
        canvas.clear();
        for (row, pixel_row) in view_plane.pixels.iter().enumerate() {
            for (col, pixel) in pixel_row.iter().enumerate() {
                canvas.set_draw_color(pixel.color);
                canvas
                    .fill_rect(Rect::new(
                        (row * scale) as i32,
                        (col * scale) as i32,
                        scale as u32,
                        scale as u32,
                    ))
                    .unwrap();
            }
        }
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }
}
