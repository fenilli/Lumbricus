mod graphics;

use graphics::Renderer;
use winit::{application::ApplicationHandler, event::WindowEvent, window::Window};

pub struct Engine {
    renderer: Option<Renderer>,
}

impl Engine {
    pub fn new() -> Self {
        Self { renderer: None }
    }
}

impl ApplicationHandler for Engine {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes().with_title("Engine"))
            .unwrap();
        self.renderer = Some(Renderer::new(window));
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let window = self.renderer.as_ref().unwrap().window();

        if window.id() == window_id {
            match event {
                WindowEvent::CloseRequested => event_loop.exit(),
                WindowEvent::RedrawRequested => {
                    self.renderer.as_mut().unwrap().render();
                }
                _ => (),
            };
        }
    }
}
