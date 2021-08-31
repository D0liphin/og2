use crate::*;

pub struct SpriteConfiguration<'a> {
    pub label: Option<&'a str>,
    pub mesh: SpriteMesh,
    pub texture: Texture,
}

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

    pub(crate) fn get_render_bundle(&self) -> RenderBundle {
        RenderBundle {
            texture_view: &self.texture.texture_view,
            sampler: &self.texture.sampler,
            vertex_buffer_contents: &self.mesh.vertex_buffer_contents(),
            index_buffer_contents: &self.mesh.index_buffer_contents(),
            matrix: &self.mesh.matrix,
        }
    }

    /// Queue an additional 2x2 linear transformation to be executed on this sprite during the shader stage.  
    ///
    /// Because this only queues a transformation, you should keep track of performance-critical geometric
    /// data yourself.
    pub fn transform(&mut self, matrix: &Matrix2x2) {
        self.mesh.matrix = Matrix3x2::compose(
            &self.mesh.matrix,
            &Matrix3x2::new(matrix.i, matrix.j, Vector2::new(0.0, 0.0)),
        );
    }

    /// Set the exact position of this sprite
    pub fn set_position(&mut self, position: Vector2) {
        self.mesh.matrix.k = position;
    }

    /// Unqueue all transformations to be exectured on this sprite, replacing them with a new one
    pub fn set_transformation(&mut self, matrix: Matrix2x2) {
        self.mesh.matrix = Matrix3x2::new(matrix.i, matrix.j, self.mesh.matrix.k);
    }
}
