use std::f32::consts::{FRAC_PI_2, PI};

pub trait Angle {
    fn rotate(self, radians: f32) -> f32;
    fn as_angle(self) -> f32;
}

fn clamp(n: f32) -> f32 {
    if n > PI {
        -2. * PI + n
    } else if n < -PI {
        PI + (n + PI)
    } else {
        n
    }
}

impl Angle for f32 {
    /// Rotates an angle `radians` radians, making sure it stays bounded.
    /// Only works with bounded angles
    fn rotate(self, radians: f32) -> f32 {
        let unbounded = self + radians;
        clamp(unbounded)
    }

    /// converts any value into a bounded angle
    fn as_angle(mut self) -> f32 {
        let loops = (self / (2. * PI)).abs().floor() * 2. * PI;
        if self.is_sign_positive() {
            self -= loops;
        } else {
            self += loops;
        }
        clamp(self)
    }
}
