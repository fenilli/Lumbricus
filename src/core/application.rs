use std::sync::Arc;

pub use winit::dpi::PhysicalSize;
use winit::{
    application::ApplicationHandler, event::WindowEvent, event_loop::EventLoop, window::Window,
};

use super::renderer::Renderer;

struct ApplicationState {
    renderer: Renderer,

    window: Arc<Window>,
}

impl ApplicationState {
    pub fn new(window: Window) -> Self {
        let window = Arc::new(window);
        let renderer = Renderer::new(window.clone());

        Self { renderer, window }
    }

    pub fn event(
        &mut self,
        event: &winit::event::WindowEvent,
        event_loop: &winit::event_loop::ActiveEventLoop,
    ) {
        match event {
            WindowEvent::RedrawRequested => self.renderer.draw(),
            WindowEvent::Resized(size) => self.renderer.resize(*size),
            WindowEvent::CloseRequested => event_loop.exit(),
            _ => (),
        }
    }

    pub fn tick(&self) {
        self.window.request_redraw();
    }
}

pub struct ApplicationDescriptor {
    pub title: &'static str,
    pub size: PhysicalSize<u16>,
}

pub struct Application {
    descriptor: ApplicationDescriptor,
    state: Option<ApplicationState>,
}

impl Application {
    pub fn run(descriptor: ApplicationDescriptor) {
        let event_loop = EventLoop::new().unwrap();
        let mut application = Self {
            descriptor,
            state: None,
        };
        _ = event_loop.run_app(&mut application);
    }
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window_attributes = Window::default_attributes()
            .with_title(self.descriptor.title)
            .with_inner_size(self.descriptor.size);

        let window = event_loop.create_window(window_attributes).unwrap();
        self.state = Some(ApplicationState::new(window));
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        self.state.as_mut().unwrap().event(&event, event_loop);
    }

    fn about_to_wait(&mut self, _: &winit::event_loop::ActiveEventLoop) {
        self.state.as_ref().unwrap().tick();
    }
}
