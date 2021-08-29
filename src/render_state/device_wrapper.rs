use crate::*;
use wgpu::{include_wgsl, util::DeviceExt};

pub(crate) struct DeviceWrapper {
    pub(crate) device: wgpu::Device,
    pub(crate) texture_bind_group_layout: wgpu::BindGroupLayout,
    pub(crate) preferred_texture_format: wgpu::TextureFormat,
}

impl DeviceWrapper {
    pub(crate) async fn new(
        window: &winit::window::Window,
    ) -> (Self, wgpu::Surface, wgpu::SurfaceConfiguration, wgpu::Queue) {
        let init_size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await
            .expect("Could not acquire adapter.");

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .await
            .expect("Could not acquire device.");

        let preferred_texture_format = surface
            .get_preferred_format(&adapter)
            .expect("Could not acquire preferred texture format.");

        let surface_configuration = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: preferred_texture_format,
            width: init_size.width,
            height: init_size.height,
            present_mode: wgpu::PresentMode::Mailbox,
        };
        surface.configure(&device, &surface_configuration);

        let texture_bind_group_layout = Self::create_bind_group_layout(&device);

        (
            Self {
                device,
                texture_bind_group_layout,
                preferred_texture_format,
            },
            surface,
            surface_configuration,
            queue,
        )
    }

    pub(crate) fn create_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Bind Group Layout Descriptor"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler {
                        comparison: false,
                        filtering: true,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: Some(RenderState::UNIFORM_BUFFER_SIZE),
                    },
                    count: None,
                },
            ],
        })
    }

    /// Creates a new BindGroup, using `self.texture_bind_group_layout`.
    pub(crate) fn create_texture_bind_group(
        &self,
        texture_view: &wgpu::TextureView,
        sampler: &wgpu::Sampler,
        uniform_buffer_contents: &[u8],
    ) -> wgpu::BindGroup {
        self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Texture Bind Group"),
            layout: &self.texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        offset: 0,
                        buffer: &self.device.create_buffer_init(
                            &wgpu::util::BufferInitDescriptor {
                                label: Some("Normalization Matrix"),
                                contents: uniform_buffer_contents,
                                usage: wgpu::BufferUsages::UNIFORM,
                            },
                        ),
                        size: Some(RenderState::UNIFORM_BUFFER_SIZE),
                    }),
                },
            ],
        })
    }

    pub(crate) fn create_render_pipeline(&self) -> wgpu::RenderPipeline {
        let render_pipeline_layout =
            self.device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[&self.texture_bind_group_layout],
                    ..Default::default()
                });

        use wgpu::include_wgsl;
        let shader_module = self
            .device
            .create_shader_module(&include_wgsl!("../wgsl/shader.wgsl"));

        self.device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader_module,
                    entry_point: "main",
                    buffers: &[wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<[f32; 4]>() as u64,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &[
                            wgpu::VertexAttribute {
                                offset: 0,
                                shader_location: 0,
                                format: wgpu::VertexFormat::Float32x2,
                            },
                            wgpu::VertexAttribute {
                                offset: std::mem::size_of::<[f32; 2]>() as u64,
                                shader_location: 1,
                                format: wgpu::VertexFormat::Float32x2,
                            },
                        ],
                    }],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader_module,
                    entry_point: "main",
                    targets: &[wgpu::ColorTargetState {
                        format: self.preferred_texture_format,
                        blend: Some(wgpu::BlendState {
                            color: wgpu::BlendComponent {
                                src_factor: wgpu::BlendFactor::SrcAlpha,
                                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                                operation: wgpu::BlendOperation::Add,
                            },
                            alpha: wgpu::BlendComponent  {
                                src_factor: wgpu::BlendFactor::One,
                                dst_factor: wgpu::BlendFactor::One,
                                operation: wgpu::BlendOperation::Add,                 
                            },
                        }),
                        write_mask: wgpu::ColorWrites::ALL,
                    }],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    polygon_mode: wgpu::PolygonMode::Fill,
                    ..Default::default()
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
            })
    }

    pub(crate) fn create_vertex_buffer(&self, contents: &[u8]) -> wgpu::Buffer {
        self.device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents,
                usage: wgpu::BufferUsages::VERTEX,
            })
    }

    pub(crate) fn create_index_buffer(&self, contents: &[u8]) -> wgpu::Buffer {
        self.device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents,
                usage: wgpu::BufferUsages::INDEX,
            })
    }

    pub(crate) fn create_command_encoder(&self) -> wgpu::CommandEncoder {
        self.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            })
    }

    pub(crate) fn create_sampler(&self) -> wgpu::Sampler {
        self.device
            .create_sampler(&wgpu::SamplerDescriptor {
                ..Default::default()
            })
    }
}
