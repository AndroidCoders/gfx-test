use sdl3::EventPump;
use sdl3::video::Window;
use sdl3::render::Canvas;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;

use crate::game_state::GameState;
#[cfg(feature = "debug-video-to-png")]
use crate::frame_capture::FrameCapture;

const SCREEN_WIDTH: u32 = 1920;
const SCREEN_HEIGHT: u32 = 1080;

pub struct App {
    canvas: Canvas<Window>,
    game_state: GameState,
    event_pump: EventPump,
    #[cfg(feature = "debug-video-to-png")]
    frame_capture: Option<FrameCapture>,
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
            #[cfg(feature = "debug-video-to-png")]
            frame_capture: Some(FrameCapture::new()),
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

            #[cfg(feature = "debug-video-to-png")]
            {
                if let Some(fc) = &mut self.frame_capture {
                    fc.capture_frame(self.game_state.frame_counter, &mut self.canvas)?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(feature = "debug-video-to-png")]
impl Drop for App {
    fn drop(&mut self) {
        if let Some(fc) = &self.frame_capture {
            fc.save_frames();
        }
    }
}
