use winit::dpi::PhysicalSize;

use crate::{rect::Rect, renderer::Renderer};

pub struct Paddle {
    pub rect: Rect,
}

impl Paddle {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            rect: Rect::new(x, y, 10, 100),
        }
    }

    pub fn collide(&mut self, window_size: PhysicalSize<u32>) {
        if self.rect.y < 0.0 {
            self.rect.y = 0.0;
        } else if self.rect.y + self.rect.height as f32 > window_size.height as f32 {
            self.rect.y = window_size.height as f32 - self.rect.height as f32;
        }
    }

    pub fn update(&mut self) {}

    pub fn draw(&self, renderer: &mut Renderer) {
        renderer.draw_rect(&self.rect);
    }
}
