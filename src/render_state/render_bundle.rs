use rand::Rng;

use crate::*;

#[derive(Debug)]
/// An object that can be rendered
pub struct RenderBundle {
    pub(crate) bind_group: wgpu::BindGroup,
    pub(crate) vertex_buffer: wgpu::Buffer,
    pub(crate) index_buffer: wgpu::Buffer,
    pub(crate) index_count: u32,
    pub(crate) z_index: ZIndex,
}

impl PartialOrd for RenderBundle {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.z_index.partial_cmp(&other.z_index)
    }
}

impl PartialEq for RenderBundle {
    fn eq(&self, other: &Self) -> bool {
        other.z_index.eq(&other.z_index)
    }
}

impl Eq for RenderBundle {
    fn assert_receiver_is_total_eq(&self) {}
}

impl Ord for RenderBundle {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        <ZIndex as Ord>::cmp(&self.z_index, &other.z_index)
    }
}

pub trait IntoRenderBundle {
    fn get_render_bundle(self, oge: &Oge) -> RenderBundle;
}

impl IntoRenderBundle for RenderBundle {
    fn get_render_bundle(self, _: &Oge) -> RenderBundle {
        self
    }
}
