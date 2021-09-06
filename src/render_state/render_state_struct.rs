use crate::*;

pub(crate) struct RenderState {
    pub(crate) surface: wgpu::Surface,
    pub(crate) surface_configuration: wgpu::SurfaceConfiguration,
    pub(crate) device_wrapper: crate::DeviceWrapper,
    pub(crate) queue: wgpu::Queue,
    pub(crate) render_pipeline: wgpu::RenderPipeline,
    pub(crate) multisampled_frame_buffer: wgpu::Texture,
    pub(crate) sample_count: u32,
}

impl RenderState {
    pub(crate) const UNIFORM_BUFFER_SIZE: std::num::NonZeroU64 = unsafe {
        std::num::NonZeroU64::new_unchecked(
            3 * (8 + 8) + // 3 x vector2 with 8 padding [[size(48)]]
            1 * (4 + 12), // 1 x f32 with 12 padding
        )
    };

    pub(crate) const INITIAL_SAMPLE_COUNT: u32 = 4;

    pub fn new(window: &winit::window::Window) -> Self {
        pollster::block_on(Self::new_async(window))
    }

    async fn new_async(window: &winit::window::Window) -> Self {
        let (device_wrapper, surface, surface_configuration, queue) =
            DeviceWrapper::new(window).await;
        let render_pipeline = device_wrapper.create_render_pipeline(Self::INITIAL_SAMPLE_COUNT);
        let multisampled_frame_buffer = {
            let inner_size = window.inner_size();
            device_wrapper.create_multisampled_frame_buffer(
                inner_size.width,
                inner_size.height,
                Self::INITIAL_SAMPLE_COUNT,
            )
        };

        Self {
            surface,
            surface_configuration,
            device_wrapper,
            queue,
            render_pipeline,
            multisampled_frame_buffer,
            sample_count: Self::INITIAL_SAMPLE_COUNT,
        }
    }

    pub(crate) fn create_image_texture(
        &self,
        label: Option<&str>,
        dynamic_image: image::DynamicImage,
    ) -> Result<wgpu::Texture> {
        use image::GenericImageView;

        let image_buffer = dynamic_image
            .as_rgba8()
            .ok_or(crate::TextureError::format(label))?;

        let dimensions = dynamic_image.dimensions();
        if dimensions == (0, 0) {
            panic!("Image dimensions must be at least (1, 1)");
        }

        self.create_image_texture_from_buffer(label, image_buffer, dimensions)
    }

    pub(crate) fn create_image_texture_from_buffer(
        &self,
        label: Option<&str>,
        image_buffer: &[u8],
        dimensions: (u32, u32),
    ) -> Result<wgpu::Texture> {
        let texture_extent_3d = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let texture = self
            .device_wrapper
            .device
            .create_texture(&wgpu::TextureDescriptor {
                label,
                size: texture_extent_3d,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            });

        self.queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            image_buffer,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(dimensions.0 << 2),
                rows_per_image: std::num::NonZeroU32::new(dimensions.1),
            },
            texture_extent_3d,
        );

        Ok(texture)
    }

    pub(crate) fn resize(&mut self, new_size: &crate::WindowDimensions) {
        self.surface_configuration.width = new_size.width;
        self.surface_configuration.height = new_size.height;
        self.recreate_surface();
    }

    pub(crate) fn recreate_surface(&mut self) {
        self.surface
            .configure(&self.device_wrapper.device, &self.surface_configuration);
        self.multisampled_frame_buffer = self.device_wrapper.create_multisampled_frame_buffer(
            self.surface_configuration.width,
            self.surface_configuration.height,
            self.sample_count,
        );
    }

    pub(crate) fn create_render_pass_resources(&self) -> Result<RenderPassResources> {
        let surface_texture = self
            .surface
            .get_current_frame()
            .or(Err(crate::RenderError::frame()))?
            .output;
        let surface_texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        Ok(RenderPassResources {
            command_encoder: self.device_wrapper.create_command_encoder(),
            _surface_texture: surface_texture,
            surface_texture_view,
            render_bundles: vec![],
            multisampled_frame_buffer_view: self
                .multisampled_frame_buffer
                .create_view(&wgpu::TextureViewDescriptor::default()),
        })
    }

    pub(crate) fn configure_render_pipeline(&mut self, config: RenderPipelineConfiguration) {
        self.sample_count = config.anti_aliasing as u32;
        self.render_pipeline = self
            .device_wrapper
            .create_render_pipeline(self.sample_count);
    }
}
