use std::sync::Arc;

use pollster::FutureExt;
use wgpu::{Buffer, Device, Instance, Queue, RenderPass, Surface, SurfaceConfiguration};
use winit::{dpi::PhysicalSize, window::Window};

use super::{
    buffer::{create_index_buffer, create_uniform_buffer, create_vertex_buffer},
    camera::Camera,
    pipeline::Pipeline,
    rect::Rect,
};

pub struct Renderer {
    surface: Surface<'static>,
    surface_config: SurfaceConfiguration,

    device: Device,
    queue: Queue,

    camera: Camera,

    index_buffer: Buffer,
    projection_buffer: Buffer,
}

impl Renderer {
    pub fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();
        let instance = Instance::new(Default::default());

        let surface = instance.create_surface(window).unwrap();
        let adapter = instance
            .request_adapter(&Default::default())
            .block_on()
            .unwrap();

        let (device, queue) = adapter
            .request_device(&Default::default(), None)
            .block_on()
            .unwrap();

        let surface_config = surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();

        surface.configure(&device, &surface_config);

        let camera = Camera::new(size);

        let index_buffer = create_index_buffer(&device, Vec::from(&[0, 1, 2, 2, 3, 0]));
        let projection_buffer =
            create_uniform_buffer(&device, camera.get_projection_matrix().as_slice().into());

        Self {
            surface,
            surface_config,

            device,
            queue,

            camera,

            index_buffer,
            projection_buffer,
        }
    }

    pub fn draw(&mut self) {
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

            self.draw_rect(&mut render_pass, &Rect::new(20.0, 20.0, 200, 200));
        }

        self.queue.submit(std::iter::once(command_encoder.finish()));
        output.present();
    }

    pub fn draw_rect(&mut self, render_pass: &mut RenderPass, rect: &Rect) {
        let pipeline = Pipeline::new(&self.device, &self.surface_config, &self.projection_buffer);

        let vertex_data = [
            // Top Left
            rect.x,
            rect.y,
            1.0,
            1.0,
            1.0,
            // Top Right
            rect.x + rect.width as f32,
            rect.y,
            1.0,
            1.0,
            1.0,
            // Bottom Right
            rect.x + rect.width as f32,
            rect.y + rect.height as f32,
            1.0,
            1.0,
            1.0,
            // Bottom Left
            rect.x,
            rect.y + rect.height as f32,
            1.0,
            1.0,
            1.0,
        ];

        let vertex_buffer = create_vertex_buffer(&self.device, vertex_data.into());

        render_pass.set_pipeline(&pipeline.pipeline);
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));

        render_pass.set_bind_group(0, &pipeline.projection_bind_group, &[]);

        render_pass.draw_indexed(0..6, 0, 0..1);
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.surface_config.width = size.width;
        self.surface_config.height = size.height;
        self.surface.configure(&self.device, &self.surface_config);
        self.camera.resize(size);
    }
}
