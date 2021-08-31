use crate::*;

#[derive(Debug)]
pub(crate) struct RenderBundle<'a> {
    pub(crate) texture_view: &'a wgpu::TextureView,
    pub(crate) sampler: &'a wgpu::Sampler,
    pub(crate) vertex_buffer_contents: &'a [u8],
    pub(crate) index_buffer_contents: &'a [u8],
    pub(crate) affine2: &'a Affine2,
}