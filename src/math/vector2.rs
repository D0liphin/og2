#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
/// Representation of 2D vectors and points.
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

// pub instance
impl Vector2 {
    /// Shorthand creation
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}
