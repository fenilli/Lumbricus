use lumbricus::{Application, ApplicationDescriptor, LifecycleHandler};

struct Game {
    counter: u32,
}

impl Game {
    pub fn new() -> Self {
        Self { counter: 0 }
    }
}

impl LifecycleHandler for Game {
    fn initialize(&mut self) {
        println!("Initialized");
    }

    fn update(&mut self) {
        self.counter += 1;
        println!("Counter: {}", self.counter);
    }

    fn shutdown(&mut self) {
        self.counter = 0;
        println!("Exit: {}", self.counter);
    }
}

fn main() {
    let game = Game::new();
    Application::run(
        ApplicationDescriptor {
            title: "My Game",
            height: 600,
            width: 800,
        },
        game,
    );
}
