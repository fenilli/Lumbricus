use paddle::Paddle;

use crate::renderer::Renderer;

mod paddle;

pub struct Game {
    player: Paddle,
}

impl Game {
    pub fn new() -> Self {
        Self {
            player: Paddle::new(20, 250),
        }
    }

    pub fn update(&mut self) {
        self.player.update();
    }

    pub fn draw(&mut self, renderer: &mut Renderer) {
        self.player.draw(renderer);
    }
}
