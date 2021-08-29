use crate::*;

pub struct Oge {
    pub(crate) window_handler: WindowHandler,
    pub(crate) render_state: RenderState,
}

impl Oge {
    pub fn create_texture(&self, config: &TextureConfiguration) -> Texture {
        Texture::new(&self.render_state, &config)
    }

    pub fn render_sprites<'a, T: IntoIterator<Item = &'a Sprite>>(&self, sprites: T) {
        let surface_texture = match self.render_state.surface.get_current_frame() {
            Err(e) => return,
            Ok(frame) => frame,
        }
        .output;
        
        {
            let surface_texture_view = surface_texture
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());

            for sprite in sprites {
                let mut render_bundle = sprite.get_render_bundle();
                let matrix = Matrix3x2::compose(&self.window_handler.matrix, render_bundle.matrix);
                render_bundle.matrix = &matrix;
                self.render_state
                    .render(&surface_texture_view, render_bundle);
            }
        }
        // done!
    }
}
