use cgmath::num_traits::Signed;

use crate::*;
use std::f32::consts::{FRAC_PI_2, PI};

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
/// Representation of 2D vectors and points.
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

// pub instance
impl Vector2 {
    /// Creates a new cartesian vector with the given unit vector scalars.
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Vector2 { x: 0., y: -0. }
    pub const ZERO: Vector2 = Vector2 { x: 0., y: 0. };

    /// Vector2 { x: 0., y: -1. }
    pub const DOWN: Vector2 = Vector2 { x: 0., y: -1. };

    /// Vector2 { x: -1., y: 0. }
    pub const LEFT: Vector2 = Vector2 { x: -1., y: 0. };

    /// Vector2 { x: 0., y: 1. }
    pub const UP: Vector2 = Vector2 { x: 0., y: 1. };

    /// Vector2 { x: 1., y: 0. }
    pub const RIGHT: Vector2 = Vector2 { x: 1., y: 0. };

    /// Create a new vector pointing in the given direction, with the given magnitude
    pub fn new_euclidean(direction: f32, magnitude: f32) -> Self {
        let direction = direction - FRAC_PI_2;
        Vector2::new(direction.cos(), -direction.sin()).scale(magnitude)
    }

    /// Returns the signed direction of this vector, with 0 being directly up
    /// and -180 / 180 being down
    pub fn direction(&self) -> f32 {
        let x_is_sign_positive = self.x.is_sign_positive();

        if self.x == 0.0 {
            return if self.y.is_sign_negative() { PI } else { 0.0 };
        } else if self.y == 0.0 {
            return if x_is_sign_positive {
                FRAC_PI_2
            } else {
                -FRAC_PI_2
            };
        }

        let angle = (self.y / self.x).atan();

        if x_is_sign_positive {
            FRAC_PI_2 - angle
        } else {
            -FRAC_PI_2 - angle
        }
    }

    /// returns the signed angle, between `from` and `to`
    pub fn angle_between(from: &Self, to: &Self) -> f32 {
        let angle = to.direction() - from.direction();
        if angle > PI {
            -(2.0 * PI - angle)
        } else if angle < -PI {
            2.0 * PI + angle
        } else {
            angle
        }
    }

    /// Returns the distance between this vector and another, `to`.
    pub fn distance_to(&self, to: &Self) -> f32 {
        ((to.x - self.x).powi(2) + (self.y - to.y).powi(2)).sqrt()
    }

    /// Returns the magnitude of this vector
    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    /// Returns the normalized version of this matrix.
    pub fn normalize(self) -> Self {
        self.scale(1. / self.magnitude())
    }

    /// Normalizes this matrix
    pub fn normalize_assign(&mut self) {
        self.scale_assign(1. / self.magnitude());
    }

    /// Returns a vector that is this vector normalized, then scaled by `magnitude`
    pub fn with_magnitude(self, magnitude: f32) -> Self {
        self.scale(magnitude / self.magnitude())
    }

    /// Returns this vectors direction, relative to another point `origin`
    pub fn relative_direction(&self, origin: &Vector2) -> f32 {
        (*self).sub(origin).direction()
    } 

    /// Rotates this vector pi / 2 radians counter clockwise, returning the result 
    pub(crate) fn rotate_90_ccw(self) -> Self  {
        Self::new(-self.y, self.x)
    }

    /// Rotates this vector pi / 2 radians clockwise, returning the result 
    pub(crate) fn rotate_90_cw(self) -> Self  {
        Self::new(self.y, -self.x)
    }

    /// Rotates this vector pi radians, returning the result
    pub(crate) fn rotate_180(self) -> Self  {
        self.scale(-1.)
    }
}

// ops
impl Vector2 {
    /// Adds this vector to another, moving `self`, and returning the result.
    pub fn add(mut self, rhs: &Self) -> Self {
        self.add_assign(rhs);
        self
    }

    /// Mutates `self`, by adding `rhs` to it.
    pub fn add_assign(&mut self, rhs: &Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }

    /// Subtracts this vector from another, moving `self`, and returning the result.
    pub fn sub(mut self, rhs: &Self) -> Self {
        self.sub_assign(rhs);
        self
    }

    /// Mutates `self`, by subtracting `rhs` from it.
    pub fn sub_assign(&mut self, rhs: &Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }

    /// Computes the dot (scalar) product of `self · rhs`, moving `self` and returning the result.
    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }

    /// Multiplies this vector by a given scalar, moving `self` and returning the result.
    pub fn scale(mut self, rhs: f32) -> Self {
        self.scale_assign(rhs);
        self
    }

    /// Mutates `self by multiplying it by `rhs`.
    pub fn scale_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }

    /// Multiplies this vector by a given matrix, moving `self` and returning the result.
    pub fn mul(mut self, rhs: &Matrix2) -> Self {
        self.mul_assign(rhs);
        self
    }

    /// Mutates `self by multiplying it by `rhs`.
    pub fn mul_assign(&mut self, rhs: &Matrix2) {
        let (x, y) = (self.x, self.y);
        self.x *= rhs.i.x;
        self.x += y * rhs.j.x;
        self.y *= rhs.j.y;
        self.y += x * rhs.i.y;
    }
}
