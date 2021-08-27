use crate::*;

#[derive(Debug, Clone, Copy, Default)]
pub struct Matrix2x2 {
    pub i: Vector2,
    pub j: Vector2,
}

// pub instance
impl Matrix2x2 {
    /// Instantiates a new `Matrix3x2` from two provided column vectors: `i` and `j`
    pub const fn new(i: Vector2, j: Vector2) -> Self {
        Self { i, j }
    }
}
