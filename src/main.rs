use pong_rust::core::application::{Application, ApplicationDescriptor, PhysicalSize};

fn main() {
    Application::run(ApplicationDescriptor {
        title: "Pong Rust",
        size: PhysicalSize::new(800, 600),
    });
}
