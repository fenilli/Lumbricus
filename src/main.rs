use std::time::{Duration, Instant};

use lumbricus::{Application, ApplicationContext, ApplicationDescriptor, LifecycleHandler};

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
    fn initialize(&mut self, _context: &ApplicationContext) {
        println!("Initialized");
    }

    fn fixed_update(&mut self, delta: f32) {
        self.fixed_update_count += 1;
        println!("fixed_update: {}", delta);
    }

    fn update(&mut self, _delta: f32) {
        if self.last_second.elapsed() >= Duration::from_secs(1) {
            println!(
                "Fixed updates in the last second: {}",
                self.fixed_update_count
            );
            self.fixed_update_count = 0;
            self.last_second = Instant::now();
        }
    }

    fn shutdown(&mut self, _context: &ApplicationContext) {}
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
