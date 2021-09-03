use crate::*;

/// Represents a line connecting two points
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Line {
    pub position: Vector2,
    pub direction: Vector2,
}

impl Line {
    /// Creates a line connecting the points `a` and `b`
    pub fn connect(a: &Vector2, b: &Vector2) -> Self {
        Self {
            position: *a,
            direction: (*b).sub(a),
        }
    }

    /// Finds the intersection point between two lines, or return `None`, if the lines are
    /// parallel or cooincident
    pub fn intersection(&self, other: &Line) -> Option<Vector2> {
        let t = {
            let den = self.direction.y * other.direction.x - self.direction.x * other.direction.y;
            if den == 0. {
                return None;
            }
            (self.direction.x * (other.position.y - self.position.y)
                - self.direction.y * (other.position.x - self.position.x))
                / den
        };
        Some(other.direction.scale(t).add(&other.position))
    }

    /// Moves all points on this line over by `shift` and returns a line representing this
    pub fn shift(&self, shift: &Vector2) -> Self {
        Self {
            position: self.position.add(&shift),
            direction: self.direction,
        }
    }
}
