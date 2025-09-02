use sdl3::rect::Rect;

pub struct GameObject {
    pub rect: Rect,
    pub velocity: (i32, i32),
}

pub struct GameState {
    pub objects: Vec<GameObject>,
    pub frame_counter: u32,
}

impl GameState {
    pub fn new() -> Self {
        let objects = vec![
            GameObject {
                rect: Rect::new(100, 100, 100, 100),
                velocity: (1, 0),
            },
            GameObject {
                rect: Rect::new(200, 200, 50, 50),
                velocity: (2, 1),
            },
            GameObject {
                rect: Rect::new(300, 300, 75, 75),
                velocity: (-1, -1),
            },
        ];
        Self {
            objects,
            frame_counter: 0,
        }
    }

    pub fn update(&mut self) {
        self.frame_counter += 1;
        for object in &mut self.objects {
            object.rect.x += object.velocity.0;
            object.rect.y += object.velocity.1;

            if object.rect.x > 1920 {
                object.rect.x = -100;
            }
            if object.rect.y > 1080 {
                object.rect.y = -100;
            }
            if object.rect.x < -100 {
                object.rect.x = 1920;
            }
            if object.rect.y < -100 {
                object.rect.y = 1080;
            }
        }
    }
}
