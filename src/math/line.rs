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
        Some(other.point_at(t))
    }

    /// Moves all points on this line over by `shift` and returns a line representing this
    pub fn shift(&self, shift: &Vector2) -> Self {
        Self {
            position: self.position.add(&shift),
            direction: self.direction,
        }
    }

    /// Gets the point on the line self(t)
    pub fn point_at(&self, t: f32) -> Vector2 {
        self.direction.scale(t).add(&self.position)
    }
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let m = self.direction.y / self.direction.x;
        let point = self.point_at(0.);
        let c = point.y - m * point.x;
        write!(f, "y = {}x + {}", m, c)
    }
}
