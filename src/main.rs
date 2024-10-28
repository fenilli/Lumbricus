use std::{
    collections::HashSet,
    time::{Duration, Instant},
};

use lumbricus::{
    Application, ApplicationContext, ApplicationDescriptor, BindingDescriptor, LifecycleHandler,
};
use winit::keyboard::KeyCode;

struct Game {
    last_second: Instant,
    fixed_update_count: u32,
}

impl Game {
    pub fn new() -> Self {
        Self {
            last_second: Instant::now(),
            fixed_update_count: 0,
        }
    }
}

impl LifecycleHandler for Game {
    fn initialize(&mut self, context: &mut ApplicationContext) {
        println!("Initialized");
        let mut jump_keys = HashSet::new();
        jump_keys.insert(KeyCode::KeyW);

        context.input.bind_action(
            "jump",
            BindingDescriptor {
                keys: jump_keys,
                ..Default::default()
            },
        );
    }

    fn fixed_update(&mut self, delta: f32, context: &ApplicationContext) {
        self.fixed_update_count += 1;
        println!("fixed_update: {}", delta);

        if context.input.is_action_pressed("jump") {
            println!("Jumped!");
        }
    }

    fn update(&mut self, _delta: f32, _context: &ApplicationContext) {
        if self.last_second.elapsed() >= Duration::from_secs(1) {
            println!(
                "Fixed updates in the last second: {}",
                self.fixed_update_count
            );
            self.fixed_update_count = 0;
            self.last_second = Instant::now();
        }
    }

    fn shutdown(&mut self, context: &mut ApplicationContext) {
        println!("Shutdown");

        context.input.clear_bindings();
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
