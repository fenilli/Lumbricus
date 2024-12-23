use winit::dpi::PhysicalSize;

use crate::{rect::Rect, renderer::Renderer};

const MOVE_SPEED: f32 = 7.0;

pub struct Paddle {
    pub rect: Rect,
}

impl Paddle {
    pub fn new(x: f32, y: f32) -> Self {
        let width = 10.0;
        let height = 100.0;

        Self {
            rect: Rect::new(x, y - (height / 2.0), width as u32, height as u32),
        }
    }

    pub fn collide(&mut self, window_size: PhysicalSize<u32>) {
        if self.rect.y < 0.0 {
            self.rect.y = 0.0;
        } else if self.rect.y + self.rect.height as f32 > window_size.height as f32 {
            self.rect.y = window_size.height as f32 - self.rect.height as f32;
        }
    }

    pub fn update(&mut self, ball_rect: &Rect) {
        if self.rect.y + (self.rect.height as f32 / 2.0) < ball_rect.y {
            self.rect.y += MOVE_SPEED;
        }

        if self.rect.y - (self.rect.height as f32 / 2.0) > ball_rect.y {
            self.rect.y -= MOVE_SPEED;
        }
    }

    pub fn draw(&self, renderer: &mut Renderer) {
        renderer.draw_rect(&self.rect);
    }
}
