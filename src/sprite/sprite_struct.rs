use rand::distributions::Uniform;

use crate::*;

pub struct SpriteConfiguration<'a> {
    pub label: Option<&'a str>,
    pub mesh: SpriteMesh,
    pub texture: &'a TextureConfiguration,
    pub z_index: ZIndex,
    pub opacity: f32,
}

impl Default for SpriteConfiguration<'_> {
    fn default() -> Self {
        SpriteConfiguration {
            label: None,
            mesh: SpriteMesh::new_rectangle(0.5, 0.5),
            texture: &DEFAULT_TEXTURE_CONFIGURATION,
            z_index: ZIndex::Specific(0),
            opacity: 1.,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord)]
/// Desribes how sprites are layered
pub enum ZIndex {
    /// Render this sprite above all other sprites. Cannot guarantee that this
    /// sprite will be rendered above other sprites with `ZIndex::AboveAll`
    AboveAll,
    /// Give this a specfiic z_index.
    Specific(u32),
    /// Always render this sprite below all other sprutes. Cannot guarantee that
    /// this sprite will be rendered below other sprites with `ZIndex::BelowAll`
    BelowAll,
}

use std::cmp::Ordering;
impl PartialOrd for ZIndex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match self {
            ZIndex::AboveAll => Ordering::Greater,
            ZIndex::BelowAll => Ordering::Less,
            ZIndex::Specific(self_layer) => match other {
                ZIndex::AboveAll => Ordering::Less,
                ZIndex::BelowAll => Ordering::Greater,
                ZIndex::Specific(other_layer) => self_layer.cmp(&other_layer),
            },
        })
    }
}

#[derive(Debug)]
pub struct Sprite {
    pub(crate) label: Option<Box<str>>,
    pub(crate) mesh: SpriteMesh,
    pub(crate) texture: Texture,
    pub(crate) z_index: ZIndex,
    pub(crate) opacity: f32,
}

impl Sprite {
    /// Constructs a new sprite from a given `SpriteConfiguration`
    pub(crate) fn new(render_state: &RenderState, config: SpriteConfiguration) -> Result<Self> {
        let mut this = Self {
            label: match config.label {
                Some(label) => Some(label.to_owned().into_boxed_str()),
                None => None,
            },
            mesh: config.mesh,
            texture: Texture::new(render_state, config.texture)?,
            z_index: config.z_index,
            opacity: 1.,
        };
        this.mesh.update_texture_coordinates(&this.texture);
        this.set_opacity(config.opacity);
        Ok(this)
    }

    pub(crate) fn get_render_bundle(&self, oge: &Oge) -> RenderBundle {
        #[repr(C)]
        pub(crate) struct UniformBufferContents {
            affine2: Affine2,
            opacity: f32,
        }

        impl UniformBufferContents {
            fn as_vec_u8(&self) -> Vec<u8> {
                let size = RenderState::UNIFORM_BUFFER_SIZE.get() as usize;
                let mut dst = Vec::<u8>::with_capacity(size);
                unsafe {
                    dst.set_len(size);
                }
                for el in dst.iter_mut() {
                    *el = 0;
                }

                for (i, vector2) in [
                    self.affine2.matrix2.i,
                    self.affine2.matrix2.j,
                    self.affine2.translation,
                ]
                .iter()
                .enumerate()
                {
                    let dst = &mut dst[i * 16] as *mut u8;
                    unsafe { std::ptr::copy(vector2 as *const Vector2 as *const u8, dst, 8) }
                }
                unsafe {
                    std::ptr::copy(
                        &self.opacity as *const f32 as *const u8,
                        &mut dst[size - 16] as *mut u8,
                        4,
                    );
                }

                dst
            }
        }

        let mut affine2 = self.mesh.affine2;
        affine2 = affine2.reverse_compose(&oge.handlers.window_handler.affine2);
        affine2
            .translation
            .mul_assign(&oge.handlers.window_handler.affine2.matrix2);

        let uniform_buffer_contents = UniformBufferContents {
            affine2,
            opacity: self.opacity,
        };

        let bind_group = oge.render_state.device_wrapper.create_texture_bind_group(
            &self.texture.texture_view,
            &self.texture.sampler,
            &uniform_buffer_contents.as_vec_u8(),
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
            z_index: self.z_index,
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

    /// Set the opacity of this sprite. Values should be from `0.` to `1.`.  
    ///
    /// Values outside of these bounds will be capped.
    pub fn set_opacity(&mut self, opacity: f32) {
        self.opacity = if opacity > 1. {
            1.
        } else if opacity < 0. {
            0.
        } else {
            opacity
        };
    }

    /// Returns the opacity of this sprite
    pub fn opacity(&self) -> f32 {
        self.opacity
    }
}
