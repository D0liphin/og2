use crate::*;
use std::slice::{Iter, IterMut};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum CurveStyle {
    /// The most basic curve style. This preserves the
    PreserveAngles,
    /// angles not preserved, but no spikes
    DoubleJointed,
    /// unimplemented
    Bezier(f32),
}

/// A `Sprite` wrapper for drawing curves and paths.
///
/// You should probably use this if you want to draw curves, instead of
/// making a sprite with `SpriteMesh::new_line()`, as this efficiently
/// modifies the mesh just as much as it needs to instead of building a new
/// one each time.
///
/// NOTE; The above is a lie. I am yet to implement this so-called "efficiency".
#[derive(Debug)]
pub struct Curve {
    pub(crate) sprite: Sprite,
    pub(crate) width: f32,
    pub(crate) points: Vec<Vector2>,
    pub(crate) mapped_points: Vec<Vector2>,
    pub(crate) style: CurveStyle,
    pub(crate) updated: bool,
}

pub struct CurveConfiguration<'a> {
    /// A label used for debugging
    pub label: Option<&'a str>,
    /// The width of this curve
    pub width: f32,
    /// The points that are used to generate this curve (must be at least 2 in length)
    pub points: Vec<Vector2>,
    /// The way the curve should be drawn based on the provided points
    pub style: CurveStyle,
    /// The texture this curve should use
    pub texture_configuration: &'a TextureConfiguration,
    /// Same as the z_index attribute on `SpriteConfiguration`
    pub z_index: ZIndex,
}

impl Curve {
    // Creates a new curve from the given points
    pub fn new(oge: &Oge, config: CurveConfiguration) -> Result<Self> {
        if config.points.len() < 2 {
            panic!("Curve must have at least 2 points");
        }
        let mapped_points = Self::map_points(&config.points, config.style);
        Ok(Curve {
            sprite: Sprite::new(&oge.render_state, SpriteConfiguration {
                label: config.label,
                mesh: SpriteMesh::new_line(config.width, &config.points),
                texture: config.texture_configuration,
                z_index: config.z_index,
            })?,
            points: config.points,
            mapped_points,
            width: config.width,
            style: config.style,
            updated: true,
        })
    }

    /// Returns the index of the joint closest to `point` as well as the distance
    /// from `point`
    pub fn nearest_joint(&self, point: &Vector2) -> (usize, f32) {
        let mut index = 0;
        let mut distance = point.distance_to(&self.points[0]);
        for (i, line_point) in self.iter_points().enumerate().skip(1) {
            let distance_to_line_point = point.distance_to(line_point);
            if distance_to_line_point < distance {
                distance = distance_to_line_point;
                index = i;
            }
        }
        (index, distance)
    }

    /// extend this line, by adding a point to the end
    pub fn push(&mut self, point: Vector2) {
        self.updated = true;
        self.points.push(point);
    }

    /// Inserts a point at the given index
    pub fn insert(&mut self, index: usize, point: Vector2) {
        self.updated = true;
        self.points.insert(index, point);
    }

    /// Removes the point at the specified index and returns it. Panics if the line becomes
    /// less than two points long.
    pub fn remove(&mut self, index: usize) -> Vector2 {
        self.updated = true;
        self.points.remove(index)
    }

    /// get a reference to the point at the specified index
    pub fn get(&self, index: usize) -> &Vector2 {
        &self.points[index]
    }

    /// gets a mutable reference to the point at the specified index
    pub fn get_mut(&mut self, index: usize) -> &mut Vector2 {
        self.updated = true;
        &mut self.points[index]
    }

    /// Returns an immutable iterator over this curve's points
    pub fn iter_points(&self) -> Iter<Vector2> {
        self.points.iter()
    }

    /// Returns a mutable iterator this curve's points
    pub fn iter_mut_points(&mut self) -> IterMut<Vector2> {
        self.updated = true;
        self.points.iter_mut()
    }

    /// Returns a reference to the points that make up this line
    pub fn points(&self) -> &Vec<Vector2> {
        &self.points
    }

    /// Returns a mutable reference to the points that make up this line
    pub fn points_mut(&mut self) -> &mut Vec<Vector2> {
        &mut self.points
    }

    /// Returns a reference to the internal sprite, for drawing.
    ///
    /// Can do a lot of calculations - be careful!
    pub fn get_sprite(&mut self) -> &Sprite {
        if self.updated {
            self.updated = false;
            self.sprite.mesh = SpriteMesh::new_line(self.width, {
                match self.style {
                    CurveStyle::PreserveAngles => &self.points,
                    CurveStyle::DoubleJointed => {
                        self.mapped_points = Self::map_points(&self.points, self.style);
                        &self.mapped_points
                    }
                    CurveStyle::Bezier(_) => unimplemented!(),
                }
            });
        }
        &self.sprite
    }

    pub(crate) fn map_points(points: &Vec<Vector2>, style: CurveStyle) -> Vec<Vector2> {
        match style {
            CurveStyle::PreserveAngles => vec![],
            CurveStyle::DoubleJointed => {
                let mut mapped_points = Vec::<Vector2>::with_capacity(points.len() * 2);
                mapped_points.push(points[0]);
                for i in 1..points.len() - 1 {
                    let prev_point = unsafe { points.get_unchecked(i - 1) };
                    let point = unsafe { points.get_unchecked(i) };
                    let next_point = unsafe { points.get_unchecked(i + 1) };
                    mapped_points.extend([
                        prev_point.sub(point).with_magnitude(0.01).add(point),
                        next_point.sub(point).with_magnitude(0.01).add(point),
                    ]);
                }
                mapped_points.push(*points.last().unwrap());
                mapped_points
            }
            CurveStyle::Bezier(_) => unimplemented!(),
        }
    }
}
