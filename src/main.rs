extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
mod aperture_color;

pub fn main() {
    let mut fullscreen = false;
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(aperture_color::get_background());
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(aperture_color::get_background());
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::F),
                    ..
                } => {
                    if !fullscreen {
                        match canvas
                            .window_mut()
                            .set_fullscreen(sdl2::video::FullscreenType::Desktop)
                        {
                            Ok(_) => fullscreen = true,
                            Err(_) => (),
                        }
                    } else {
                        match canvas
                            .window_mut()
                            .set_fullscreen(sdl2::video::FullscreenType::Off)
                        {
                            Ok(_) => fullscreen = false,
                            Err(_) => (),
                        }
                    }
                }
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
