use crate::*;

/// A `Sprite` wrapper for drawing curves and paths. 
///
/// You should probably use this if you want to draw curves, instead of
/// making a sprite with `SpriteMesh::new_line()`, as this efficiently
/// modifies the mesh just as much as it needs to instead of building a new 
/// one each time.
#[derive(Debug)]
pub struct Curve {
    pub(crate) sprite: Sprite,
    pub points: Vec<Vector2>,
}

pub struct CurveConfiguration {
    pub width: f32,
    pub points: Vec<Vector2>,
    pub texture_configuration: TextureConfiguration,
}

impl Curve {
    // Creates a new curve from the given points
    pub fn new(oge: &Oge, width: f32, points: Vec<Vector2>) -> Self {
        Curve {
            sprite: Sprite::new(SpriteConfiguration {
                label: Some("Curve"),
                mesh: SpriteMesh::new_line(width, &points),
                texture: oge.create_texture(),
            }),
            points: (),
        }
    }

    // extend this line, by adding a point to the end
    pub fn push(point: Vector2) -> Self {
        todo!()
    }

    /// Returns a reference to the internal sprite, for drawing.
    pub fn sprite(&self) -> &Sprite {
        &self.sprite
    }
}
