use std::sync::Arc;

use pollster::FutureExt;
use wgpu::{Device, Instance, Queue, Surface};
use winit::window::Window;

pub struct Graphics {
    pub surface: Surface<'static>,
    pub device: Device,
    pub queue: Queue,

    pub window: Arc<Window>,
}

impl Graphics {
    pub fn new(window: Window) -> Self {
        let window = Arc::new(window);
        let instance = Instance::new(wgpu::InstanceDescriptor {
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone()).unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                ..Default::default()
            })
            .block_on()
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    ..Default::default()
                },
                None,
            )
            .block_on()
            .unwrap();

        Self {
            surface,
            device,
            queue,
            window,
        }
    }
}
