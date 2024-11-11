use nalgebra::Matrix4;
use winit::dpi::PhysicalSize;

pub struct Camera {
    projection: Matrix4<f32>,
    size: PhysicalSize<u32>,
}

impl Camera {
    pub fn new(size: PhysicalSize<u32>) -> Self {
        Self {
            projection: Matrix4::new_orthographic(
                0.0,
                size.width as f32,
                size.height as f32,
                0.0,
                -1.0,
                1.0,
            ),
            size,
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.size = size;
        self.projection =
            Matrix4::new_orthographic(0.0, size.width as f32, size.height as f32, 0.0, -1.0, 1.0);
    }

    pub fn get_projection_matrix(&self) -> Matrix4<f32> {
        self.projection
    }
}
