use paddle::Paddle;
use player::Player;

use crate::{input::Input, renderer::Renderer};

mod paddle;
mod player;

pub struct Game {
    player: Player,
    enemy: Paddle,
}

impl Game {
    pub fn new() -> Self {
        Self {
            player: Player::new(20, 250),
            enemy: Paddle::new(770, 250),
        }
    }

    pub fn input(&mut self, input: &Input) {
        self.player.input(input);
    }

    pub fn update(&mut self) {
        self.player.update();
        self.enemy.update();
    }

    pub fn draw(&mut self, renderer: &mut Renderer) {
        self.player.draw(renderer);
        self.enemy.draw(renderer);
    }
}
