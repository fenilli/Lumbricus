use wgpu::{util::DeviceExt, Buffer, Device, Queue};

pub fn create_vertex_buffer(device: &Device, data: Vec<f32>) -> wgpu::Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(&data),
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
    })
}

pub fn create_index_buffer(device: &Device, data: Vec<u16>) -> wgpu::Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(&data),
        usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
    })
}

pub fn create_uniform_buffer(device: &Device, data: Vec<f32>) -> wgpu::Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(&data),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    })
}

pub fn write_uniform_buffer(queue: &Queue, buffer: &Buffer, data: Vec<f32>) {
    queue.write_buffer(buffer, 0, bytemuck::cast_slice(&data));
}
