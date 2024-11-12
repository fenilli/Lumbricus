use nalgebra::Vector2;
use winit::dpi::PhysicalSize;

use crate::{rect::Rect, renderer::Renderer};

const BASE_SPEED: f32 = 2.0;

pub struct Ball {
    rect: Rect,
    speed: f32,
    direction: Vector2<f32>,
}

impl Ball {
    pub fn new(window_size: PhysicalSize<u32>) -> Self {
        let dx = rand::random::<f32>();
        let dy = rand::random::<f32>();

        Self {
            rect: Rect::new(
                window_size.width as f32 / 2.0,
                window_size.height as f32 / 2.0,
                10,
                10,
            ),
            speed: BASE_SPEED,
            direction: Vector2::new(dx, dy).normalize(),
        }
    }

    pub fn update(&mut self) {
        self.rect.x += self.direction.x * self.speed;
        self.rect.y += self.direction.y * self.speed;
    }

    pub fn draw(&self, renderer: &mut Renderer) {
        renderer.draw_rect(&self.rect);
    }
}
