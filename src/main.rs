use lumbricus::{Application, ApplicationDescriptor, LifecycleHandler};
use winit::keyboard::KeyCode;

struct Game {}

impl Game {
    pub fn new() -> Self {
        Self {}
    }
}

impl LifecycleHandler for Game {
    fn initialize(&mut self) {
        println!("Initialized");
    }

    fn input(&mut self, input: &lumbricus::Input) {
        if input.is_key_pressed(KeyCode::KeyW) {
            println!("Pressed!");
        }

        if input.is_key_released(KeyCode::KeyW) {
            println!("Released!");
        }
    }

    fn fixed_update(&mut self, _delta: f32) {
        // println!("fixed_update: {}", delta);
    }

    fn update(&mut self, _delta: f32) {
        // println!(
        //     "Fixed updates in the last second: {}",
        //     self.fixed_update_count
        // );
    }

    fn shutdown(&mut self) {
        println!("Shutdown");
    }
}

fn main() {
    let game = Game::new();
    Application::run(
        ApplicationDescriptor {
            title: "My Game",
            height: 600,
            width: 800,
            fixed_time: 60,
        },
        game,
    );
}
