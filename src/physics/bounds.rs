/// Represents a 2-dimensional region spanning from `bottom_left` to `top_right`.  
/// The bounding box represented by `Bounds` is always aligned with the coordinate axes.
#[derive(Default, Debug, Clone, Copy)]
pub struct Bounds {
    pub bottom_left: crate::Vector2,
    pub top_right: crate::Vector2,
}

impl Bounds {
    /// Returns the (signed) width of this bounding box
    pub fn width(&self) -> f32 {
        self.top_right.x - self.bottom_left.x
    }

    /// Returns the (signed) height of this bounding box
    pub fn height(&self) -> f32 {
        self.top_right.y - self.bottom_left.y
    }

    /// Returns a vector representing the center of this bounding box
    pub fn center(&self) -> crate::Vector2 {
        crate::Vector2 {
            x: self.width() / 2.0 + self.bottom_left.x,
            y: self.height() / 2.0 + self.bottom_left.y,
        }
    }
}
