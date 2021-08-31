/// Represents a color. Values are floats 0.0 - 1.0
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl Color {
    /// Creates a new `Color` object with the specified components
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color { r, g, b, a }
    }

    pub const BLACK: Self = Self::new(0., 0., 0., 1.);
    
    pub const WHITE: Self = Self::new(1., 1., 1., 1.,);

    pub(crate) fn as_u8(float: f32) -> u8 {
        (float * 255.).round() as u8
    }

    /// Returns a byte array representing this color. 
    pub fn as_rgba8(&self) -> [u8; 4] {
        [
            Color::as_u8(self.r),
            Color::as_u8(self.g),
            Color::as_u8(self.b),
            Color::as_u8(self.a),
        ]
    }
}
