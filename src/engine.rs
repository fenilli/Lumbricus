use crate::{graphics::Graphics, input::Input, square::Square, timer::Timer};

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

pub struct Engine {
    graphics: Option<Graphics>,
    timer: Timer,
    input: Input,

    square: Option<Square>,
}

impl Engine {
    fn new() -> Self {
        Self {
            graphics: None,
            timer: Timer::new(),
            input: Input::new(),

            square: None,
        }
    }

    pub fn run() {
        let mut engine = Engine::new();

        let event_loop = EventLoop::new().unwrap();
        _ = event_loop.run_app(&mut engine);
    }
}

impl ApplicationHandler for Engine {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attrs = Window::default_attributes().with_title("Lumbricus");
        let window = event_loop.create_window(window_attrs).unwrap();
        let graphics = Graphics::new(window);
        self.square = Some(Square::new(&graphics));
        self.graphics = Some(graphics);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        self.input.update(&event);

        match event {
            WindowEvent::RedrawRequested => {
                let graphics = self.graphics.as_ref().unwrap();
                let _dt = self.timer.tick();

                match graphics.surface.get_current_texture() {
                    Ok(output) => {
                        let view = output
                            .texture
                            .create_view(&wgpu::TextureViewDescriptor::default());

                        let mut encoder = graphics.device.create_command_encoder(
                            &wgpu::CommandEncoderDescriptor { label: None },
                        );

                        self.square.as_mut().unwrap().render(&mut encoder, &view);

                        graphics.queue.submit(std::iter::once(encoder.finish()));
                        output.present();
                    }
                    _ => {}
                }

                // println!("DT: {}, FPS: {}", dt, self.timer.get_fps());
            }
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(physical_size) => {
                let graphics = self.graphics.as_mut().unwrap();

                graphics.rezise(physical_size);
            }
            _ => {}
        };
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        self.input.clear();
        self.graphics.as_ref().unwrap().window().request_redraw();
    }
}
