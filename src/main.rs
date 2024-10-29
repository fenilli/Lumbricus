use lumbricus::{Application, ApplicationContext, ApplicationDescriptor, LifecycleHandler};
use winit::keyboard::KeyCode;

struct Game {}

impl Game {
    pub fn new() -> Self {
        Self {}
    }
}

impl LifecycleHandler for Game {
    fn initialize(&mut self, _context: &mut ApplicationContext) {
        println!("Initialized");
    }

    fn fixed_update(&mut self, _delta: f32, _context: &ApplicationContext) {
        // println!("fixed_update: {}", delta);
    }

    fn update(&mut self, _delta: f32, context: &ApplicationContext) {
        // println!(
        //     "Fixed updates in the last second: {}",
        //     self.fixed_update_count
        // );

        if context.input.is_key_pressed(KeyCode::KeyW) {
            println!("Pressed!");
        }

        if context.input.is_key_released(KeyCode::KeyW) {
            println!("Released!");
        }
    }

    fn shutdown(&mut self, _context: &mut ApplicationContext) {
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
