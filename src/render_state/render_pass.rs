use crate::*;

pub(crate) struct RenderPassResources {
    pub(crate) command_encoder: wgpu::CommandEncoder,
    // _surface_texture must not be dropped before any views that have been created from it
    pub(crate) surface_texture_view: wgpu::TextureView,
    pub(crate) _surface_texture: wgpu::SurfaceTexture,
    pub(crate) render_bundles: Vec<RenderBundle>,
    pub(crate) multisampled_frame_buffer_view: wgpu::TextureView,
}

impl RenderPassResources {
    pub(crate) fn finish(self, render_state: &RenderState) {
        render_state
            .queue
            .submit(std::iter::once(self.command_encoder.finish()));
        drop(self.surface_texture_view);
    }
}

pub struct RenderPass<'a> {
    pub(crate) _color_attachments: [wgpu::RenderPassColorAttachment<'a>; 1],
    pub(crate) render_pass: wgpu::RenderPass<'a>,
    pub(crate) render_bundles: &'a mut Vec<RenderBundle>,
}

impl<'a> RenderPass<'a> {
    pub(crate) fn draw_render_bundles(mut self, render_state: &'a RenderState) {
        self.render_bundles.sort_unstable();
        for render_bundle in self.render_bundles.iter() {
            self.render_pass.set_pipeline(&render_state.render_pipeline);
            self.render_pass
                .set_bind_group(0, &render_bundle.bind_group, &[]);
            self.render_pass
                .set_vertex_buffer(0, render_bundle.vertex_buffer.slice(..));
            self.render_pass.set_index_buffer(
                render_bundle.index_buffer.slice(..),
                wgpu::IndexFormat::Uint16,
            );
            self.render_pass
                .draw_indexed(0..render_bundle.index_count, 0, 0..1);
        }
    }
}
