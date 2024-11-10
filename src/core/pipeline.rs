use wgpu::{Buffer, Device, SurfaceConfiguration};

pub struct Pipeline {
    pub pipeline: wgpu::RenderPipeline,
    pub projection_bind_group: wgpu::BindGroup,
}

impl Pipeline {
    pub fn new(
        device: &Device,
        surface_config: &SurfaceConfiguration,
        projection_buffer: &Buffer,
    ) -> Self {
        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/shader.wgsl").into()),
        });

        let vertex_buffer_layout = wgpu::VertexBufferLayout {
            array_stride: 5 * 4, // Size of Quad vertexes
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x2,
                    shader_location: 0,
                    offset: 0,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    shader_location: 1,
                    offset: 2 * 4,
                },
            ],
        };

        let vertex_state = wgpu::VertexState {
            module: &shader_module,
            entry_point: "vs_main",
            buffers: &[vertex_buffer_layout],
            compilation_options: Default::default(),
        };

        let fragment_state = wgpu::FragmentState {
            module: &shader_module,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: surface_config.format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        };

        let projection_buffer_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&projection_buffer_layout],
            push_constant_ranges: &[],
        });

        let projection_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &projection_buffer_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: projection_buffer.as_entire_binding(),
            }],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: vertex_state,
            fragment: Some(fragment_state),
            primitive: wgpu::PrimitiveState::default(),
            multisample: wgpu::MultisampleState::default(),
            depth_stencil: None,
            cache: None,
            multiview: None,
        });

        Self {
            pipeline,
            projection_bind_group,
        }
    }
}
