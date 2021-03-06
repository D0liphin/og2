use crate::*;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
/// A 3-column, 2-row matrix
pub struct Affine2 {
    pub matrix2: Matrix2,
    pub translation: Vector2,
}

impl Default for Affine2 {
    fn default() -> Self {
        Affine2 {
            matrix2: Matrix2::default(),
            translation: Vector2::ZERO,
        }
    }
}

impl Affine2 {
    /// Instantiates a new `Affine2` from the three provided column vectors:
    /// `i`, `j` and `k`.
    pub const fn new(i: Vector2, j: Vector2, k: Vector2) -> Self {
        Self {
            matrix2: Matrix2::new(i, j),
            translation: k,
        }
    }
}

impl Affine2 {
    /// Composes this matrix with another, returning the result. The resulting matrix
    /// Will have the same effect as applying `rhs` **followed by** `self`. See
    /// `Matrix2::then(self, lhs: &Matrix2) -> Matrix2` for the reverse.
    pub fn compose(mut self, rhs: &Self) -> Self {
        self.compose_assign(rhs);
        self
    }

    /// Sets this matrix to the result of the composition `self ◦ rhs`
    pub fn compose_assign(&mut self, rhs: &Self) {
        self.matrix2.compose_assign(&rhs.matrix2);
        self.translation.add_assign(&rhs.translation);
    }

    /// Composes matrices backwards (`lhs ◦ self`) see `Matrix2::compose(self, rhs: &Self)`
    /// for more details.
    pub fn reverse_compose(self, lhs: &Self) -> Self {
        lhs.compose(&self)
    }
}
