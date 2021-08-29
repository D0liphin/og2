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

impl std::ops::Add<Vector2> for Vector2 {
    type Output = Self;

    fn add(mut self, rhs: Vector2) -> Self::Output {
        self.x += rhs.x;
        self.y += rhs.y;
        self
    }
}

impl std::ops::Mul<f32> for Vector2 {
    type Output = Self;

    fn mul(mut self, rhs: f32) -> Self::Output {
        self.x *= rhs;
        self.y *= rhs;
        self
    }
}