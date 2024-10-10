use no_name::Engine;
use winit::event_loop::{ControlFlow, EventLoop};

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut engine = Engine::new();

    event_loop.run_app(&mut engine).unwrap();
}
