use crate::*;
use std::path::PathBuf;

#[derive(Clone, Copy, Debug)]
pub enum TextureProjectionMethod {
    ScaleToFit = 0,
    /// Should be used for textures that are created from a single color.
    /// Provides a faster creation than `ScaleToFit` but is functionally
    /// identical for `Color` texture sources.
    SingleColor = 1,
}

pub enum TextureSource {
    Path(PathBuf),
    Bytes(&'static [u8]),
    Color(Color),
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
/// Texel mixing mode when sampling between texels
pub enum FilterMode {
    /// Texture pixels become blocky up close
    Point = 0,
    /// Texture samples are averaged
    Bilinear = 1,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
/// How edges should be handled in texture addressing
pub enum AddressMode {
    /// Textures are clamped to the borders
    Clamp = 0,
    /// Textures wrap around with a repeating pattern
    Wrap = 1,
    /// Textures wrap around with a repeating pattern, that is mirrored each time
    Mirror = 2,
}

/// Describes how a `Texture` should be configured
pub struct TextureConfiguration {
    pub source: TextureSource,
    /// The projection method that should be used for this texture
    pub projection_method: TextureProjectionMethod,
    /// The filter mode to be used for this texture's sampler
    pub filter_mode: FilterMode,
    /// The address mode to be used for this texture's sampler
    pub address_mode: AddressMode,
}

impl Default for TextureConfiguration {
    fn default() -> Self {
        TextureConfiguration {
            source: TextureSource::Color(Color::WHITE),
            projection_method: TextureProjectionMethod::ScaleToFit,
            filter_mode: FilterMode::Bilinear,
            address_mode: AddressMode::Clamp,
        }
    }
}

impl TextureConfiguration {
    /// Shorthand for
    /// ````rs
    /// TextureConfiguration {
    ///     source: TextureSource::Color(color),
    ///     projection_method: TextureProjectionMethod::SingleColor,
    ///     ..Default::default()
    /// }
    /// ```
    pub fn color(color: Color) -> Self {
        TextureConfiguration {
            source: TextureSource::Color(color),
            projection_method: TextureProjectionMethod::SingleColor,
            ..Default::default()
        }
    }
}

#[derive(Debug)]
pub struct Texture {
    pub(crate) texture: wgpu::Texture,
    pub(crate) texture_view: wgpu::TextureView,
    pub(crate) sampler: wgpu::Sampler,
    pub(crate) projection_method: TextureProjectionMethod,
    pub(crate) dimensions: (u32, u32),
}

// pub(crate)
impl Texture {
    pub(crate) fn load_image(path_buf: &PathBuf) -> Result<image::DynamicImage> {
        image::io::Reader::open(path_buf)
            .or(Err(crate::TextureError::open(&path_buf)))?
            .decode()
            .or(Err(crate::TextureError::decode(&path_buf)))
    }

    pub(crate) fn create_from_dynamic_image(
        render_state: &RenderState,
        dynamic_image: image::DynamicImage,
    ) -> Result<(wgpu::Texture, wgpu::TextureView, (u32, u32))> {
        use image::GenericImageView;

        let dimensions = dynamic_image.dimensions();
        let texture = render_state.create_image_texture(None, dynamic_image)?;
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        Ok((texture, texture_view, dimensions))
    }

    pub(crate) fn new(render_state: &RenderState, config: &TextureConfiguration) -> Result<Self> {
        let (texture, texture_view, dimensions) = match &config.source {
            TextureSource::Path(path_buf) => {
                let dynamic_image = Texture::load_image(&path_buf)?;
                Self::create_from_dynamic_image(render_state, dynamic_image)?
            }
            TextureSource::Bytes(bytes) => {
                let dynamic_image =
                    image::load_from_memory(bytes).or(Err(crate::TextureError::load_bytes()))?;
                Self::create_from_dynamic_image(render_state, dynamic_image)?
            }
            TextureSource::Color(color) => {
                let dimensions = (1, 1);
                let texture = render_state.create_image_texture_from_buffer(
                    None,
                    &color.as_rgba8(),
                    dimensions,
                )?;
                let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
                (texture, texture_view, dimensions)
            }
        };

        let filter_mode = unsafe { crate::util::cast_enum(config.filter_mode) };
        let address_mode = unsafe { crate::util::cast_enum(config.address_mode) };

        Ok(Self {
            texture,
            texture_view,
            sampler: render_state
                .device_wrapper
                .create_sampler(filter_mode, address_mode),
            projection_method: config.projection_method,
            dimensions,
        })
    }

    /// Returns the dimensions of this texture in pixels
    pub fn dimensions(&self) -> (u32, u32) {
        self.dimensions
    }
}
