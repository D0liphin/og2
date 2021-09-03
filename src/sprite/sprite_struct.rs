use crate::*;

pub struct SpriteConfiguration<'a> {
    pub label: Option<&'a str>,
    pub mesh: SpriteMesh,
    pub texture: Texture,
}

#[derive(Debug)]
pub struct Sprite {
    label: Option<Box<str>>,
    mesh: SpriteMesh,
    texture: Texture,
}

impl Sprite {
    /// Constructs a new sprite from a given `SpriteConfiguration`
    pub fn new(config: SpriteConfiguration) -> Self {
        let mut this = Self {
            label: match config.label {
                Some(label) => Some(label.to_owned().into_boxed_str()),
                None => None,
            },
            mesh: config.mesh,
            texture: config.texture,
        };
        this.mesh.update_texture_coordinates(&this.texture);
        this
    }

    pub(crate) fn get_render_bundle(&self, oge: &Oge) -> RenderBundle {
        
        let bind_group = oge.render_state.device_wrapper.create_texture_bind_group(
            &self.texture.texture_view,
            &self.texture.sampler,
            &{
                let mut affine2 = self.mesh.affine2;
                affine2 = affine2.reverse_compose(&oge.handlers.window_handler.affine2);
                affine2.translation.mul_assign(&oge.handlers.window_handler.affine2.matrix2);
                affine2.create_raw_buffer()
            },
        );
        RenderBundle {
            vertex_buffer: oge
                .render_state
                .device_wrapper
                .create_vertex_buffer(&self.mesh.vertex_buffer_contents()),
            index_buffer: oge
                .render_state
                .device_wrapper
                .create_index_buffer(&self.mesh.index_buffer_contents()),
            index_count: self.mesh.indices.len() as u32,
            bind_group,
        }
    }

    /// Queue an additional 2x2 linear transformation to be executed on this sprite during the shader stage.  
    ///
    /// Because this only queues a transformation, you should keep track of performance-critical data
    /// yourself.
    pub fn transform(&mut self, matrix: &Matrix2) {
        self.mesh
            .affine2
            .compose_assign(&Affine2::new(matrix.i, matrix.j, Vector2::new(0.0, 0.0)));
    }

    /// Set the exact position of this sprite.
    ///
    /// Note this position is only enqueued, not immediately applied.
    pub fn set_position(&mut self, position: Vector2) {
        self.mesh.affine2.translation = position;
    }

    /// Unqueue all transformations to be exectured on this sprite, replacing them with a new one
    pub fn set_transformation(&mut self, matrix: Matrix2) {
        self.mesh.affine2 = Affine2::new(matrix.i, matrix.j, self.mesh.affine2.translation);
    }
}
