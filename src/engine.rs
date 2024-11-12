use std::sync::Arc;

use pollster::FutureExt;
use wgpu::{Device, Instance, Queue, Surface, SurfaceConfiguration};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::{KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    window::Window,
};

use crate::{game::Game, input::Input};

use super::renderer::Renderer;

pub struct EngineState {
    surface: Surface<'static>,
    surface_config: SurfaceConfiguration,
    device: Arc<Device>,
    queue: Arc<Queue>,

    renderer: Renderer,
    input: Input,

    game: Game,

    window: Arc<Window>,
}

impl EngineState {
    pub fn new(window: Window) -> Self {
        let window = Arc::new(window);
        let size = window.inner_size();
        let instance = Instance::new(Default::default());

        let surface = instance.create_surface(window.clone()).unwrap();
        let adapter = instance
            .request_adapter(&Default::default())
            .block_on()
            .unwrap();

        let (device, queue) = adapter
            .request_device(&Default::default(), None)
            .block_on()
            .unwrap();

        let mut surface_config = surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();
        surface_config.format = wgpu::TextureFormat::Bgra8UnormSrgb;

        let arc_device = Arc::new(device);
        let arc_queue = Arc::new(queue);
        let renderer = Renderer::new(arc_device.clone(), arc_queue.clone(), size);

        Self {
            surface,
            surface_config,
            device: arc_device,
            queue: arc_queue,

            renderer,
            input: Input::new(),

            game: Game::new(window.inner_size()),

            window,
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        if size.width <= 0 || size.height <= 0 {
            return;
        }

        self.surface_config.width = size.width;
        self.surface_config.height = size.height;
        self.surface.configure(&self.device, &self.surface_config);
        self.renderer.resize(size);
    }

    pub fn input(&mut self, event: &KeyEvent) {
        self.input.clear();
        self.input.update(event);

        self.game.input(&self.input);
    }

    pub fn draw(&mut self) {
        self.game.update();

        let output = self.surface.get_current_texture().unwrap();
        let view = output.texture.create_view(&Default::default());

        let mut command_encoder = self.device.create_command_encoder(&Default::default());

        {
            let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.75,
                            g: 0.5,
                            b: 0.25,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });

            self.renderer.frame_start();

            self.game.draw(&mut self.renderer);

            self.renderer.frame_end(&mut render_pass);
        }

        self.queue.submit(std::iter::once(command_encoder.finish()));
        self.window.pre_present_notify();
        output.present();
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }
}

pub struct Engine {
    window_state: Option<EngineState>,
}

impl Engine {
    pub fn run() {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
        let mut application = Self { window_state: None };
        _ = event_loop.run_app(&mut application);
    }

    fn create_window(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes()
            .with_title("Pong Rust")
            .with_inner_size(PhysicalSize::new(800, 600));

        let window = event_loop.create_window(window_attributes).unwrap();
        self.window_state = Some(EngineState::new(window));
    }
}

impl ApplicationHandler for Engine {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.create_window(event_loop);
    }

    fn window_event(
        &mut self,
        _: &winit::event_loop::ActiveEventLoop,
        _: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let window_state = match &mut self.window_state {
            None => return,
            Some(window_state) => window_state,
        };

        match event {
            WindowEvent::Resized(size) => window_state.resize(size),
            WindowEvent::CloseRequested => self.window_state = None,
            WindowEvent::RedrawRequested => window_state.draw(),
            WindowEvent::KeyboardInput { event, .. } => window_state.input(&event),
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        match &self.window_state {
            None => event_loop.exit(),
            Some(window_state) => window_state.request_redraw(),
        }
    }
}
