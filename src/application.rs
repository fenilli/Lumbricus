use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::EventLoop,
    window::{Window, WindowAttributes},
};

use crate::{Clock, Input};

pub struct AppDescriptor {
    pub title: &'static str,
    pub height: u32,
    pub width: u32,
}

pub struct AppContext {
    pub clock: Clock,
    pub input: Input,
    pub window: Window,
}

pub trait AppLifecycleHandler {
    fn booted(&mut self, context: &mut AppContext);
    fn running(&mut self, context: &mut AppContext);
    fn exiting(&mut self, context: &mut AppContext);
}

pub struct Application<H: AppLifecycleHandler> {
    descriptor: AppDescriptor,
    context: Option<AppContext>,
    handler: H,
}

impl<H: AppLifecycleHandler> Application<H> {
    pub fn run(descriptor: AppDescriptor, handler: H) {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

        let mut application = Application {
            descriptor,
            context: None,
            handler,
        };
        _ = event_loop.run_app(&mut application);
    }
}

impl<H: AppLifecycleHandler> ApplicationHandler for Application<H> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.context.is_some() {
            return;
        }

        let window_attrs = WindowAttributes::default()
            .with_title(self.descriptor.title)
            .with_inner_size(PhysicalSize {
                width: self.descriptor.width,
                height: self.descriptor.height,
            });

        let window = event_loop.create_window(window_attrs).unwrap();

        self.context = Some(AppContext {
            clock: Clock::new(),
            input: Input::new(),
            window,
        });
        if let Some(context) = &mut self.context {
            self.handler.booted(context);
        }
    }

    fn about_to_wait(&mut self, _: &winit::event_loop::ActiveEventLoop) {
        if let Some(context) = &mut self.context {
            context.input.clear();
            context.window.request_redraw();
        }
    }

    fn exiting(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        if let Some(context) = &mut self.context {
            self.handler.exiting(context);
        }

        self.context = None;
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if self.context.is_none() {
            return;
        }

        if let Some(context) = &mut self.context {
            match event {
                WindowEvent::KeyboardInput { event, .. } => {
                    context.input.update(&event);
                }
                WindowEvent::RedrawRequested => {
                    context.clock.tick();
                    self.handler.running(context);
                }
                WindowEvent::CloseRequested => {
                    event_loop.exit();
                }
                _ => {}
            }
        };
    }
}
