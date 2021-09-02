use crate::*;

pub(crate) struct RenderState {
    pub(crate) surface: wgpu::Surface,
    pub(crate) surface_configuration: wgpu::SurfaceConfiguration,
    pub(crate) device_wrapper: crate::DeviceWrapper,
    pub(crate) queue: wgpu::Queue,
    pub(crate) render_pipeline: wgpu::RenderPipeline,
}

impl RenderState {
    pub(crate) const UNIFORM_BUFFER_SIZE: std::num::NonZeroU64 =
        unsafe { std::num::NonZeroU64::new_unchecked(48) };

    pub fn new(window: &winit::window::Window) -> Self {
        pollster::block_on(Self::new_async(window))
    }

    async fn new_async(window: &winit::window::Window) -> Self {
        let (device_wrapper, surface, surface_configuration, queue) =
            DeviceWrapper::new(window).await;

        let bind_group = device_wrapper.create_texture_bind_group(
            &{
                device_wrapper
                    .device
                    .create_texture(&wgpu::TextureDescriptor {
                        label: Some("Blank Texture"),
                        size: wgpu::Extent3d::default(),
                        mip_level_count: 1,
                        sample_count: 1,
                        dimension: wgpu::TextureDimension::D2,
                        format: wgpu::TextureFormat::Rgba8UnormSrgb,
                        usage: wgpu::TextureUsages::TEXTURE_BINDING,
                    })
                    .create_view(&wgpu::TextureViewDescriptor::default())
            },
            &{
                device_wrapper
                    .device
                    .create_sampler(&wgpu::SamplerDescriptor::default())
            },
            &[0; RenderState::UNIFORM_BUFFER_SIZE.get() as usize],
        );

        let render_pipeline = device_wrapper.create_render_pipeline();

        Self {
            surface,
            surface_configuration,
            device_wrapper,
            queue,
            render_pipeline,
        }
    }

    pub(crate) fn create_image_texture(
        &self,
        label: Option<&str>,
        dynamic_image: image::DynamicImage,
    ) -> Result<wgpu::Texture, OgeError> {
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
    ) -> Result<wgpu::Texture, OgeError> {
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
                // This is safe since we checked earlier if this is (0, 0)
                bytes_per_row: Some(unsafe {
                    std::num::NonZeroU32::new_unchecked(dimensions.0 << 2)
                }),
                rows_per_image: Some(unsafe { std::num::NonZeroU32::new_unchecked(dimensions.1) }),
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
            .configure(&self.device_wrapper.device, &self.surface_configuration)
    }

    pub(crate) fn create_render_pass_resources(&self) -> Result<RenderPassResources, OgeError> {
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
            surface_texture,
            surface_texture_view,
            render_bundles: vec![],
        })
    }

    // pub(crate) fn render(&self, render_target: &wgpu::TextureView, render_bundle: &RenderBundle) {
    //     let mut command_encoder = self.device_wrapper.create_command_encoder();

    //     {
    //         let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
    //             label: None,
    //             color_attachments: &[wgpu::RenderPassColorAttachment {
    //                 view: &render_target,
    //                 resolve_target: None,
    //                 ops: wgpu::Operations {
    //                     load: wgpu::LoadOp::Clear(wgpu::Color {
    //                         r: 1.0,
    //                         g: 1.0,
    //                         b: 1.0,
    //                         a: 1.0,
    //                     }),
    //                     store: true,
    //                 },
    //             }],
    //             depth_stencil_attachment: None,
    //         });

    //         render_pass.set_pipeline(&self.render_pipeline);
    //         render_pass.set_bind_group(0, &render_bundle.bind_group, &[]);
    //         render_pass.set_vertex_buffer(0, render_bundle.vertex_buffer.slice(..));
    //         render_pass.set_index_buffer(
    //             render_bundle.index_buffer.slice(..),
    //             wgpu::IndexFormat::Uint16,
    //         );
    //         render_pass.draw_indexed(0..render_bundle.index_count, 0, 0..1);
    //     }

    //     self.queue.submit(std::iter::once(command_encoder.finish()));
    // }
}
