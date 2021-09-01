// use crate::*;

// pub(crate) struct RenderPassResources {
//     pub(crate) command_encoder: wgpu::CommandEncoder,
//     // DO NOT FLIP WITH BELOW
//     pub(crate) surface_texture_view: wgpu::TextureView,
//     pub(crate) surface_texture: wgpu::SurfaceTexture,
// }

// pub struct RenderPass<'a> {
//     pub(crate) color_attachments: [wgpu::RenderPassColorAttachment<'a>; 1],
//     pub(crate) render_pass: wgpu::RenderPass<'a>,
// }

// impl<'a> RenderPass<'a> {
//     pub(crate) fn render(&mut self, render_state: &'a RenderState, render_bundle: &'a RenderBundle) {
//         let bind_group = render_state.device_wrapper.create_texture_bind_group(
//             &render_bundle.texture_view,
//             &render_bundle.sampler,
//             &render_bundle.affine2.create_raw_buffer(),
//         );

//         self.render_pass.set_pipeline(&render_state.render_pipeline);
//         self.render_pass.set_bind_group(0, &bind_group, &[]);
//         self.render_pass
//             .set_vertex_buffer(0, render_bundle.vertex_buffer.slice(..));
//         self.render_pass
//             .set_index_buffer(render_bundle.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
//         self.render_pass.draw_indexed(
//             0..render_bundle.index_count,
//             0,
//             0..1,
//         );
//     }
// }
