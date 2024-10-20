mod framerate;

use framerate::Framerate;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

struct Engine {
    window: Option<Window>,
    framerate: Framerate,
}

impl Engine {
    fn new() -> Self {
        Self {
            window: None,
            framerate: Framerate::new(),
        }
    }

    fn run() {
        let mut engine = Engine::new();

        let event_loop = EventLoop::new().unwrap();
        _ = event_loop.run_app(&mut engine);
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
                let dt = self.framerate.tick();
                let fps = self.framerate.get_frames_per_second();
                println!("DT: {} - FPS: {}", dt, fps);
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
    Engine::run();
}
