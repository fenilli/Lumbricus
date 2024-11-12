use ball::Ball;
use paddle::Paddle;
use player::Player;
use winit::dpi::PhysicalSize;

use crate::{input::Input, renderer::Renderer};

mod ball;
mod paddle;
mod player;

pub struct Game {
    player: Player,
    enemy: Paddle,
    ball: Ball,

    window_size: PhysicalSize<u32>,
}

impl Game {
    pub fn new(window_size: PhysicalSize<u32>) -> Self {
        Self {
            player: Player::new(20.0, 250.0),
            enemy: Paddle::new(770.0, 250.0),
            ball: Ball::new(
                window_size.width as f32 / 2.0,
                window_size.height as f32 / 2.0,
            ),

            window_size,
        }
    }

    pub fn input(&mut self, input: &Input) {
        self.player.input(input);
    }

    pub fn update(&mut self) {
        self.ball.update();
        self.ball.collide_world(self.window_size);
        // Player
        self.ball.collide_rect(&self.player.rect);
        // Enemy
        self.ball.collide_rect(&self.enemy.rect);

        self.player.update();
        self.player.collide(self.window_size);

        self.enemy.update();
        self.enemy.collide(self.window_size);
    }

    pub fn draw(&mut self, renderer: &mut Renderer) {
        self.player.draw(renderer);
        self.ball.draw(renderer);
        self.enemy.draw(renderer);
    }
}
