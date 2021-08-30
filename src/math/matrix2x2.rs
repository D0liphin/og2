use crate::*;

#[derive(Debug, Clone, Copy, Default)]
pub struct Matrix2x2 {
    pub i: Vector2,
    pub j: Vector2,
}

// pub instance
impl Matrix2x2 {
    /// Returns
    /// ```
    /// Mat2x2 {
    ///     i: Vector2 { x: 1.0, y: 0.0 },
    ///     j: Vector2 { x: 0.0, y: 1.0 },
    /// }
    /// ```
    pub const fn new() -> Self {
        Matrix2x2 {
            i: Vector2 { x: 1.0, y: 0.0 },
            j: Vector2 { x: 0.0, y: 1.0 },
        }
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

    /// Composes matrices backwards (`lhs ◦ self`)
    pub fn then(&mut self, lhs: Matrix2x2) {
        self.i = Vector2::new(lhs.i.x, lhs.i.y) * self.i.x + Vector2::new(lhs.j.x, lhs.j.y) * self.i.y;
        self.j = Vector2::new(lhs.i.x, lhs.i.y) * self.j.x + Vector2::new(lhs.j.x, lhs.j.y) * self.j.y;
    }
}
