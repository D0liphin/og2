use std::path::PathBuf;

#[derive(Clone, Copy, Debug)]
pub enum TextureProjectionMethod {
    ScaleToFit = 0,
}

/// Describes how a `Texture` should be configured
pub struct TextureConfiguration {
    /// The path of the image that should be used for this texture
    pub path: PathBuf,
    /// The projection method that should be used for this texture
    pub projection_method: TextureProjectionMethod,
}

pub struct Texture {
    pub(crate) texture: wgpu::Texture,
    pub(crate) texture_view: wgpu::TextureView,
    pub(crate) sampler: wgpu::Sampler,
    pub(crate) projection_method: TextureProjectionMethod,
    pub(crate) dimensions: (u32, u32),
}

// pub(crate)
impl Texture {
    pub(crate) fn load_image(path_buf: &PathBuf) -> image::DynamicImage {
        image::io::Reader::open(path_buf)
            .expect(&format!("Could not open image {:?}", path_buf))
            .decode()
            .expect(&format!("Could not decode image {:?}", path_buf))
    }

    pub(crate) fn new(render_state: &crate::RenderState, config: &TextureConfiguration) -> Self {
        use image::GenericImageView;

        let dynamic_image = Texture::load_image(&config.path);
        let dimensions = dynamic_image.dimensions();
        let texture = render_state.create_image_texture(config.path.to_str(), dynamic_image);
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        Self {
            texture,
            texture_view,
            sampler: render_state.device_wrapper.create_sampler(),
            projection_method: config.projection_method,
            dimensions,
        }
    }

    /// Returns the dimensions of this texture in pixels
    pub fn dimensions(&self) -> (u32, u32) {
        self.dimensions
    }
}
