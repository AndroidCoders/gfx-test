extern crate sdl3;

pub mod game_state;
pub mod renderer;

use sdl3::pixels::Color;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;

fn main() -> Result<(), String> {
    let sdl_context = sdl3::init().map_err(|e| e.to_string())?; // TODO: Error handling
    let video_subsystem = sdl_context.video().map_err(|e| e.to_string())?; // TODO: Error handling

    let window = video_subsystem.window(
        "gfx-test",
        1920, // TODO: Use constants for screen dimensions
        1080
    )
    .fullscreen() // Set to fullscreen
    .build().map_err(|e| e.to_string())?; // TODO: Error handling

    let mut canvas = window.into_canvas().map_err(|e| e.to_string())?; // TODO: Error handling

    canvas.set_draw_color(Color::RGB(0, 0, 0)); // Black background

    let mut event_pump = sdl_context.event_pump().map_err(|e| e.to_string())?; // TODO: Error handling

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        // Clear the screen
        canvas.clear();
        // Present the changes to the screen
        canvas.present();
    }

    Ok(())
}
