use std::time::Duration;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

struct Engine {
    window: Option<Window>,
    last_frame_time: std::time::Instant,
    last_fps_check: std::time::Instant,
    frame_counter: u32,
}

impl Engine {
    fn new() -> Self {
        Self {
            window: None,
            last_frame_time: std::time::Instant::now(),
            last_fps_check: std::time::Instant::now(),
            frame_counter: 0,
        }
    }
}

impl ApplicationHandler for Engine {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attrs = Window::default_attributes().with_title("Lumbricus");
        let window = event_loop.create_window(window_attrs).unwrap();
        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::RedrawRequested => {
                let now = std::time::Instant::now();
                let delta_time = now - self.last_frame_time;
                self.last_frame_time = now;

                self.frame_counter += 1;

                if now - self.last_fps_check >= Duration::from_secs(1) {
                    println!("Delta Time: {:?}", delta_time);
                    println!("FPS: {}", self.frame_counter);
                    self.frame_counter = 0;
                    self.last_fps_check = now;
                }
            }
            WindowEvent::CloseRequested => event_loop.exit(),
            _ => {}
        };
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        self.window.as_ref().unwrap().request_redraw();
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut engine = Engine::new();
    _ = event_loop.run_app(&mut engine);
}
