use std::fs;
use image::{ColorType, ImageError};
use sdl3::render::Canvas;
use sdl3::video::Window;

const SCREEN_WIDTH: u32 = 1920;
const SCREEN_HEIGHT: u32 = 1080;
const MAX_CAPTURED_FRAMES: usize = 10;

pub struct FrameCapture {
    captured_frames: Vec<(u32, Vec<u8>)>,
}

impl FrameCapture {
    pub fn new() -> Self {
        Self {
            captured_frames: Vec::with_capacity(MAX_CAPTURED_FRAMES),
        }
    }

    pub fn capture_frame(&mut self, frame_counter: u32, canvas: &mut Canvas<Window>) -> Result<(), String> {
        if self.captured_frames.len() < MAX_CAPTURED_FRAMES &&
           (frame_counter == 1 || (frame_counter % 100 == 0 && frame_counter <= 900))
        {
            let mut surface = canvas.read_pixels(None)?;
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
            self.captured_frames.push((frame_counter, rgba_pixels));
        }
        Ok(())
    }

    pub fn save_frames(&self) {
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
