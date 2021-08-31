use cgmath::num_traits::Signed;

use crate::*;
use std::f32::consts::{FRAC_PI_2, PI};

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

    /// Create a new vector pointing in the given direction, with the given magnitude
    pub fn new_euclidean(direction: f32, magnitude: f32) -> Self {
        let direction = direction - FRAC_PI_2;
        Vector2::new(direction.cos(), -direction.sin()) * magnitude
    }

    /// Returns the signed direction of this vector, with 0 being directly up
    /// and -180 / 180 being down
    pub fn direction(&self) -> f32 {
        let x_is_sign_positive = self.x.is_sign_positive();

        if self.x == 0.0 {
            return if self.y.is_sign_negative() { PI } else { 0.0 };
        } else if self.y == 0.0 {
            return if x_is_sign_positive { FRAC_PI_2 } else { -FRAC_PI_2 };
        }

        let angle = (self.y / self.x).atan();

        if x_is_sign_positive {
            FRAC_PI_2 - angle
        } else {
            -FRAC_PI_2 - angle
        }
    }

    /// returns the signed angle, between `from` and `to`
    pub fn angle(from: Self, to: Self) -> f32 {
        let angle = to.direction() - from.direction();
        if angle > PI {
            -(2.0 * PI - angle)
        } else if angle < -PI {
            2.0 * PI + angle
        } else {
            angle
        }
    }

    /// Returns the distance between two vectors 
    pub fn distance(a: &Self, b: &Self) -> f32 {
        ((b.x - a.x).powi(2) + (a.y - b.y).powi(2)).sqrt()
    }

    /// Returns the magnitude of this vector
    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl std::ops::Add<Vector2> for Vector2 {
    type Output = Self;

    fn add(mut self, rhs: Vector2) -> Self::Output {
        self += rhs;
        self
    }
}

impl std::ops::AddAssign<Vector2> for Vector2 {
    fn add_assign(&mut self, rhs: Vector2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::Sub<Vector2> for Vector2 {
    type Output = Self;

    fn sub(mut self, rhs: Vector2) -> Self::Output {
        self -= rhs;
        self
    }
}

impl std::ops::SubAssign<Vector2> for Vector2 {
    fn sub_assign(&mut self, rhs: Vector2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl std::ops::Mul<f32> for Vector2 {
    type Output = Self;

    fn mul(mut self, rhs: f32) -> Self::Output {
        self *= rhs;
        self
    }
}

impl std::ops::MulAssign<f32> for Vector2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl std::ops::Mul<Matrix2x2> for Vector2 {
    type Output = Self;

    fn mul(self, matrix: Matrix2x2) -> Self::Output {
        matrix.i * self.x + matrix.j * self.y
    }
}

impl std::ops::MulAssign<Matrix2x2> for Vector2 {
    fn mul_assign(&mut self, matrix: Matrix2x2) {
        *self = *self * matrix;
    }
}

impl std::ops::Mul<Matrix3x2> for Vector2 {
    type Output = Self;

    fn mul(self, matrix: Matrix3x2) -> Self::Output {
        (matrix.i * self.x + matrix.j * self.y) + matrix.k
    }
}
