use sdl3::rect::Rect;

pub struct GameState {
    pub box_rect: Rect,
    pub frame_counter: u32,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            box_rect: Rect::new(100, 100, 100, 100),
            frame_counter: 0,
        }
    }

    pub fn update(&mut self) {
        self.frame_counter += 1;
        self.box_rect.x += 1;
        if self.box_rect.x > 1920 {
            self.box_rect.x = -100;
        }
    }
}
