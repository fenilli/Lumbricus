use std::sync::Arc;

use wgpu::{Buffer, Device, Queue, RenderPass};
use winit::dpi::PhysicalSize;

use super::{
    buffer::{
        create_index_buffer, create_uniform_buffer, create_vertex_buffer, write_uniform_buffer,
    },
    camera::Camera,
    pipeline::Pipeline,
    rect::Rect,
};

pub struct Renderer {
    device: Arc<Device>,
    queue: Arc<Queue>,

    camera: Camera,

    projection_buffer: Buffer,
    pipeline: Pipeline,

    vertex_data: Vec<[f32; 5 * 4]>,
}

impl Renderer {
    pub fn new(device: Arc<Device>, queue: Arc<Queue>, size: PhysicalSize<u32>) -> Self {
        let camera = Camera::new(size);

        let projection_buffer =
            create_uniform_buffer(&device, camera.get_projection_matrix().as_slice().into());

        let pipeline = Pipeline::new(&device, &projection_buffer);

        Self {
            device,
            queue,

            camera,

            projection_buffer,
            pipeline,

            vertex_data: Vec::new(),
        }
    }

    pub fn frame_start(&mut self) {
        self.vertex_data.clear();

        write_uniform_buffer(
            &self.queue,
            &self.projection_buffer,
            self.camera.get_projection_matrix().as_slice().into(),
        );
    }

    pub fn draw_rect(&mut self, rect: &Rect) {
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

        self.vertex_data.push(vertex_data);
    }

    pub fn frame_end(&mut self, render_pass: &mut RenderPass) {
        render_pass.set_pipeline(&self.pipeline.pipeline);
        render_pass.set_bind_group(0, &self.pipeline.projection_bind_group, &[]);

        for vertex in &self.vertex_data {
            let vertex_buffer = create_vertex_buffer(&self.device, vertex.into());
            let index_buffer = create_index_buffer(&self.device, Vec::from(&[0, 1, 2, 2, 3, 0]));

            render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));

            render_pass.draw_indexed(0..6, 0, 0..1);
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.camera.resize(size);
    }
}
