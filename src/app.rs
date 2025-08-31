use std::fs;
use sdl3::EventPump;
use sdl3::video::Window;
use sdl3::render::Canvas;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use image::ColorType;


use crate::game_state::GameState;

const SCREEN_WIDTH: u32 = 1920;
const SCREEN_HEIGHT: u32 = 1080;
const MAX_CAPTURED_FRAMES: usize = 10;

pub struct App {
    canvas: Canvas<Window>,
    game_state: GameState,
    event_pump: EventPump,
    captured_frames: Vec<(u32, Vec<u8>)>, // Stores (frame_counter, pixel_data)
}

impl App {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let sdl_context = sdl3::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem.window(
            "gfx-test",
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
        )
        .fullscreen()
        .build()?;

        let canvas = window.into_canvas();
        let game_state = GameState::new();
        let event_pump = sdl_context.event_pump()?;

        Ok(Self {
            canvas,
            game_state,
            event_pump,
            captured_frames: Vec::with_capacity(MAX_CAPTURED_FRAMES),
        })
    }

    pub fn run(&mut self) -> Result<(), String> {
        'running: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    _ => {}
                }
            }

            self.game_state.update();

            self.canvas.set_draw_color(Color::RGB(0, 0, 0));
            self.canvas.clear();
            self.canvas.set_draw_color(Color::RGB(255, 255, 255));
            self.canvas.fill_rect(self.game_state.box_rect).map_err(|e| e.to_string())?;
            self.canvas.present();

            // Capture frames at specific intervals for debugging
            let frame_to_capture = self.game_state.frame_counter;
            if self.captured_frames.len() < MAX_CAPTURED_FRAMES &&
               (frame_to_capture == 1 || (frame_to_capture % 100 == 0 && frame_to_capture <= 900))
            {
                let mut surface = self.canvas.read_pixels(None).map_err(|e| e.to_string())?;
                let bytes_per_pixel_source = 3; // Assuming RGB24 from the canvas

                let mut rgba_pixels = Vec::with_capacity((SCREEN_WIDTH * SCREEN_HEIGHT * 4) as usize);

                surface.with_lock_mut(|pixels_slice| {
                    for y in 0..SCREEN_HEIGHT as usize {
                        let row_start = y * (SCREEN_WIDTH as usize * bytes_per_pixel_source);
                        for x in 0..SCREEN_WIDTH as usize {
                            let pixel_start = row_start + (x * bytes_per_pixel_source);
                            rgba_pixels.push(pixels_slice[pixel_start]);     // R
                            rgba_pixels.push(pixels_slice[pixel_start + 1]); // G
                            rgba_pixels.push(pixels_slice[pixel_start + 2]); // B
                            rgba_pixels.push(255); // A (opaque)
                        }
                    }
                });
                self.captured_frames.push((frame_to_capture, rgba_pixels));
            }
        }
        Ok(())
    }
}

impl Drop for App {
    fn drop(&mut self) {
        println!("Saving captured frames...");
        let output_dir = "output";
        if let Err(e) = fs::create_dir_all(output_dir) {
            eprintln!("Error creating output directory {}: {}", output_dir, e);
            return;
        }

        for (frame_counter, buffer) in &self.captured_frames {
            let filename = format!("{}/frame_{:04}.png", output_dir, frame_counter);
            match image::save_buffer(&filename, buffer, SCREEN_WIDTH, SCREEN_HEIGHT, ColorType::Rgba8) {
                Ok(_) => println!("Successfully saved frame to {}", filename),
                Err(e) => eprintln!("Error saving frame {}: {}", filename, e),
            }
        }
    }
}