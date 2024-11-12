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
}

impl Game {
    pub fn new(window_size: PhysicalSize<u32>) -> Self {
        Self {
            player: Player::new(20.0, 250.0),
            enemy: Paddle::new(770.0, 250.0),
            ball: Ball::new(window_size),
        }
    }

    pub fn input(&mut self, input: &Input) {
        self.player.input(input);
    }

    pub fn update(&mut self) {
        self.player.update();
        self.ball.update();
        self.enemy.update();
    }

    pub fn draw(&mut self, renderer: &mut Renderer) {
        self.player.draw(renderer);
        self.ball.draw(renderer);
        self.enemy.draw(renderer);
    }
}
