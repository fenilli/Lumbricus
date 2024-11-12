use nalgebra::Vector2;
use winit::dpi::PhysicalSize;

use crate::{rect::Rect, renderer::Renderer};

const BASE_SPEED: f32 = 5.0;
const MAX_BOUNCES: u32 = 8;

pub struct Ball {
    rect: Rect,
    speed: f32,
    bounces: u32,
    direction: Vector2<f32>,
    window_size: PhysicalSize<u32>,
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
            bounces: 0,
            speed: BASE_SPEED,
            direction: Vector2::new(dx, dy).normalize(),
            window_size,
        }
    }

    pub fn update(&mut self) {
        self.check_window_collision();

        self.rect.x += self.direction.x * (self.speed + self.bounces as f32);
        self.rect.y += self.direction.y * (self.speed + self.bounces as f32);
    }

    pub fn draw(&self, renderer: &mut Renderer) {
        renderer.draw_rect(&self.rect);
    }

    fn bounced(&mut self) {
        if self.bounces < MAX_BOUNCES {
            self.bounces += 1;
        }
    }

    fn check_window_collision(&mut self) {
        if self.rect.x <= 0.0 {
            self.rect.x = 0.0;
            self.direction.x = -self.direction.x;
            self.bounced();
        } else if self.rect.x + self.rect.width as f32 >= self.window_size.width as f32 {
            self.rect.x = self.window_size.width as f32 - self.rect.width as f32;
            self.direction.x = -self.direction.x;
            self.bounced();
        }

        if self.rect.y <= 0.0 {
            self.rect.y = 0.0;
            self.direction.y = -self.direction.y;
            self.bounced();
        } else if self.rect.y + self.rect.height as f32 >= self.window_size.height as f32 {
            self.rect.y = self.window_size.height as f32 - self.rect.height as f32;
            self.direction.y = -self.direction.y;
            self.bounced();
        }
    }
}
