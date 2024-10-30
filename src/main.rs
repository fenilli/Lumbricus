use lumbricus::{AppDescriptor, AppLifecycleHandler, Application};
use winit::keyboard::KeyCode;

struct Game {}

impl Game {
    pub fn new() -> Self {
        Self {}
    }
}

impl AppLifecycleHandler for Game {
    fn booted(&mut self, _context: &mut lumbricus::AppContext) {
        println!("booted");
    }

    fn running(&mut self, context: &mut lumbricus::AppContext) {
        // println!("running: {}", context.clock.get_delta_time());

        if context.input.is_key_pressed(KeyCode::KeyW) {
            println!("Pressed W");
        }

        if context.input.is_key_released(KeyCode::KeyW) {
            println!("Released W");
        }
    }

    fn exiting(&mut self, _context: &mut lumbricus::AppContext) {
        println!("exiting");
    }
}

fn main() {
    let game = Game::new();
    Application::run(
        AppDescriptor {
            title: "My Game",
            height: 600,
            width: 800,
        },
        game,
    );
}
