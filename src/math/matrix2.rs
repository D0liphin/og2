use crate::*;

#[derive(Debug, Clone, Copy)]
pub struct Matrix2 {
    pub i: Vector2,
    pub j: Vector2,
}

impl Default for Matrix2 {
    /// Returns
    /// ```
    /// Mat2x2 {
    ///     i: Vector2 { x: 1.0, y: 0.0 },
    ///     j: Vector2 { x: 0.0, y: 1.0 },
    /// }
    /// ```
    fn default() -> Self {
        Matrix2 {
            i: Vector2::RIGHT,
            j: Vector2::UP,
        }
    }
}

impl Matrix2 {
    /// Creates a new `Matrix2` from the given column vectors
    pub const fn new(i: Vector2, j: Vector2) -> Self {
        Self { i, j }
    }

    /// Creates a matrix that represents a rotation of `angle` radians
    pub fn rotation(angle: f32) -> Self {
        let sin_theta = angle.sin();
        let cos_theta = angle.cos();
        Self {
            i: Vector2::new(cos_theta, -sin_theta),
            j: Vector2::new(sin_theta, cos_theta),
        }
    }

    /// Creates a new matrix that scales a given vector by `stretch_x` in the x-direction
    /// and `stretch_y` in the y-direction.
    pub const fn stretch(stretch_x: f32, stretch_y: f32) -> Self {
        Self {
            i: Vector2::new(stretch_x, 0.0),
            j: Vector2::new(0.0, stretch_y),
        }
    }

    /// Creates a new matrix that scales a given vector by `stretch_x` in the x-direction
    /// and `stretch_y` in the y-direction.
    pub const fn scale(factor: f32) -> Self {
        Self::stretch(factor, factor)
    }

    /// Creates a new matrix that sheers in the +- x-direction by `shift`
    pub const fn sheer_x(shift: f32) -> Self {
        Self {
            i: Vector2::new(1.0, 0.0),
            j: Vector2::new(shift, 1.0),
        }
    }

    /// Creates a new matrix that sheers in the +- y-direction by `shift`
    pub const fn sheer_y(shift: f32) -> Self {
        Self {
            i: Vector2::new(1.0, shift),
            j: Vector2::new(0.0, 1.0),
        }
    }

    /// Composes this matrix with another, returning the result. The resulting matrix
    /// Will have the same effect as applying `rhs` **followed by** `self`. See
    /// `Matrix2::then(self, lhs: &Matrix2) -> Matrix2` for the reverse. 
    pub fn compose(mut self, rhs: &Self) -> Self {
        self.compose_assign(rhs);
        self
    }
    
    /// Sets this matrix to the result of the composition `self ◦ rhs`
    pub fn compose_assign(&mut self, rhs: &Self) {
        *self = Matrix2 {
            i: rhs.i.mul(self),
            j: rhs.j.mul(self),
        };
    }

    /// Composes matrices backwards (`lhs ◦ self`) see `Matrix2::compose(self, rhs: &Self)` 
    /// for more details.
    pub fn reverse_compose(self, lhs: &Self) -> Self {
        lhs.compose(&self)
    }
}
