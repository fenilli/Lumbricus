use crate::{rect::Rect, renderer::Renderer};

pub struct Paddle {
    rect: Rect,
}

impl Paddle {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            rect: Rect::new(x, y, 10, 100),
        }
    }

    pub fn update(&mut self) {
        self.rect.y += 1;
    }

    pub fn draw(&self, renderer: &mut Renderer) {
        renderer.draw_rect(&self.rect);
    }
}
