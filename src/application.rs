use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::EventLoop,
    window::{Window, WindowAttributes},
};

use crate::{input::Input, FrameTimer, LifecycleHandler};

pub struct ApplicationDescriptor {
    pub title: &'static str,
    pub height: u32,
    pub width: u32,
    pub fixed_time: u32,
}

pub struct ApplicationContext {
    pub frame_timer: FrameTimer,
    pub input: Input,
    window: Window,
}

enum ApplicationState {
    Uninitialized(ApplicationDescriptor),
    Initialized,
    Exited,
}

pub struct Application<H: LifecycleHandler> {
    context: Option<ApplicationContext>,
    state: ApplicationState,
    handler: H,
}

impl<H: LifecycleHandler> Application<H> {
    pub fn run(descriptor: ApplicationDescriptor, handler: H) {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

        let mut application = Application {
            context: None,
            state: ApplicationState::Uninitialized(descriptor),
            handler,
        };
        _ = event_loop.run_app(&mut application);
    }
}

impl<H: LifecycleHandler> ApplicationHandler for Application<H> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        match &self.state {
            ApplicationState::Uninitialized(descriptor) => {
                let window_attrs = WindowAttributes::default()
                    .with_title(descriptor.title)
                    .with_inner_size(PhysicalSize {
                        width: descriptor.width,
                        height: descriptor.height,
                    });

                let window = event_loop.create_window(window_attrs).unwrap();
                self.context = Some(ApplicationContext {
                    frame_timer: FrameTimer::new(descriptor.fixed_time),
                    input: Input::new(),
                    window,
                });
                self.handler.initialize();
                self.state = ApplicationState::Initialized;
            }
            _ => (),
        };
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match self.state {
            ApplicationState::Initialized => {
                let context = self.context.as_mut().unwrap();
                context.input.update(&event);

                match event {
                    WindowEvent::RedrawRequested => {
                        self.handler.input(&context.input);

                        context.frame_timer.tick();
                        self.handler.update(context.frame_timer.get_delta_time());

                        while context.frame_timer.should_do_fixed_tick() {
                            self.handler
                                .fixed_update(context.frame_timer.get_fixed_delta_time());
                        }
                    }
                    WindowEvent::CloseRequested => {
                        self.handler.shutdown();
                        self.state = ApplicationState::Exited;
                        event_loop.exit();
                    }
                    _ => {}
                };
            }
            _ => (),
        };
    }

    fn about_to_wait(&mut self, _: &winit::event_loop::ActiveEventLoop) {
        match &self.state {
            ApplicationState::Initialized => {
                let context = self.context.as_mut().unwrap();
                context.input.clear();
                context.window.request_redraw();
            }
            _ => (),
        };
    }
}
