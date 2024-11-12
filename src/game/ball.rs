use nalgebra::Vector2;
use winit::dpi::PhysicalSize;

use crate::{physics::collide_rects, rect::Rect, renderer::Renderer};

const BASE_SPEED: f32 = 5.0;
const MAX_BOUNCES: u32 = 8;

pub struct Ball {
    rect: Rect,
    speed: f32,
    bounces: u32,
    direction: Vector2<f32>,
}

impl Ball {
    pub fn new(x: f32, y: f32) -> Self {
        let dx = rand::random::<f32>();
        let dy = rand::random::<f32>();

        Self {
            rect: Rect::new(x, y, 10, 10),
            bounces: 0,
            speed: BASE_SPEED,
            direction: Vector2::new(dx, dy).normalize(),
        }
    }

    pub fn collide_world(&mut self, window_size: PhysicalSize<u32>) {
        let mut collide = false;

        if self.rect.x <= 0.0 {
            self.rect.x = 0.0;
            self.direction.x = -self.direction.x;
            collide = true;
        } else if self.rect.x + self.rect.width as f32 >= window_size.width as f32 {
            self.rect.x = window_size.width as f32 - self.rect.width as f32;
            self.direction.x = -self.direction.x;
            collide = true;
        }

        if self.rect.y <= 0.0 {
            self.rect.y = 0.0;
            self.direction.y = -self.direction.y;
            collide = true;
        } else if self.rect.y + self.rect.height as f32 >= window_size.height as f32 {
            self.rect.y = window_size.height as f32 - self.rect.height as f32;
            self.direction.y = -self.direction.y;
            collide = true;
        }

        if collide {
            self.bounced();
        }
    }

    pub fn collide_rect(&mut self, target: &Rect) {
        if collide_rects(&self.rect, target) {
            self.direction.x = -self.direction.x;
            self.bounced();
        }
    }

    pub fn update(&mut self) {
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
}
