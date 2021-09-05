use crate::*;
use std::f32::consts::{FRAC_PI_2, PI};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct VertexInput {
    position: Vector2,
    texture_coordinates: Vector2,
}

impl VertexInput {
    pub fn new(position: Vector2) -> Self {
        Self {
            position,
            texture_coordinates: Vector2::ZERO,
        }
    }
}

#[derive(Debug, Clone)]
/// A Mesh that contains texture coordinates
pub struct SpriteMesh {
    pub(crate) vertices: Vec<VertexInput>,
    pub(crate) indices: Vec<u16>,
    pub(crate) affine2: Affine2,
}

impl SpriteMesh {
    pub fn new_rectangle(width: f32, height: f32) -> Self {
        let (x, y) = (width / 2.0, height / 2.0);

        Self {
            vertices: vec![
                VertexInput::new(Vector2::new(-x, -y)),
                VertexInput::new(Vector2::new(x, -y)),
                VertexInput::new(Vector2::new(x, y)),
                VertexInput::new(Vector2::new(-x, y)),
            ],
            indices: vec![
                0, 1, 2, 0, 2, 3, /* padding (though I don't think we need this) */ 0, 0,
            ],
            affine2: Affine2::default(),
        }
    }

    /// Creates a new elipse approximation, with `detail` vertices. Panics if `detail`
    /// is less than 3
    pub fn new_elipse(a: f32, b: f32, detail: u16) -> Self {
        let matrix = Matrix2::rotation(-2. * PI / detail as f32);
        let mut vector = Vector2::UP;

        let mut vertices: Vec<VertexInput> = Vec::with_capacity(detail as usize);
        vertices.push(VertexInput::new(vector));
        let mut indices: Vec<u16> = Vec::with_capacity({
            let capacity = (detail - 2) * 3;
            if capacity.is_power_of_two() {
                capacity
            } else {
                capacity + 1
            }
        } as usize);

        for i in 1..detail {
            vector.mul_assign(&matrix);
            vertices.push(VertexInput::new(vector));
            indices.push(0);
            indices.push(i);
            indices.push(i + 1);
        }

        Self {
            vertices,
            indices,
            affine2: Affine2 {
                matrix2: Matrix2::stretch(a, b),
                translation: Vector2::ZERO,
            },
        }
    }

    pub(crate) fn new_line(width: f32, points: &[Vector2]) -> Self {
        if points.len() < 2 {
            panic!("Cannot create a line using fewer than 2 points");
        }

        let half_width = 0.5 * width;
        let mut vertices = Vec::<VertexInput>::with_capacity(points.len() * 2);
        let mut indices = Vec::<u16>::with_capacity(points.len() * 6);

        // Returns the left and right translations required to move from `point` to a point on the
        // line mesh. Left and right being relative to relative_point.relative_direction(&point)
        let get_left_right_shifts = |point: &Vector2, relative_point: &Vector2| {
            let left_shift = point
                .sub(&relative_point)
                .with_magnitude(half_width)
                .rotate_90_ccw();
            let right_shift = left_shift.rotate_180();
            (left_shift, right_shift)
        };

        // for each point after the first [0], find a line either side of it that is half_width
        // away from it, angled away from the previous point
        let mut lines = Vec::<(Line, Line)>::with_capacity(points.len() * 2 - 1);
        for (prev_point_index, point) in points.iter().skip(1).enumerate() {
            let prev = unsafe { points.get_unchecked(prev_point_index) };
            let (shift_left, shift_right) = get_left_right_shifts(point, &prev);
            let line = Line::connect(point, &prev);
            lines.push((line.shift(&shift_left), line.shift(&shift_right)));
        }

        // The first point is a special case, so here we get the direction relative to the second point [1]
        {
            let point_0 = points[0];
            let (left_shift, right_shift) = get_left_right_shifts(&points[1], &point_0);
            vertices.extend([
                VertexInput::new(left_shift.add(&point_0)),
                VertexInput::new(right_shift.add(&point_0)),
            ]);
            indices.extend([0, 1, 2, 1, 3, 2]);
        }

        // Although it is not possible for lines to be parallel, it is possible (and even likely) that
        // some will be coincident. If multiple points lie on the same line, we needn't generate
        // `VertexInput`s for them, and so we increment `skips` so that our index buffer still points to
        // the correct vertices.
        let mut skips = 0;
        for next_line_index in 1..points.len() - 1 {
            let line_index = next_line_index - 1;
            // I have extensively checked this and verified that this does not, under all scenarios result
            // in undefined behaviour. The performance gain is not substantial, but can be noticeable
            // for very detailed paths.
            let (left_line, right_line) = unsafe { lines.get_unchecked(line_index) };
            let (next_left_line, next_right_line) = unsafe { lines.get_unchecked(next_line_index) };

            let vector_1 = Line::intersection(left_line, next_left_line);
            if vector_1.is_some() {
                let vector_2 = Line::intersection(right_line, next_right_line);
                vertices.extend([
                    VertexInput::new(vector_1.unwrap()),
                    VertexInput::new(vector_2.unwrap()),
                ]);
                let i = ((line_index as u16) << 1) - skips;
                indices.extend([i, i + 1, i + 2, i + 1, i + 3, i + 2]);
            } else {
                skips += 2;
            }
        }

        {
            // len - 1 points to the last vertex and len - 2 points to the penultimate vertex
            // we always draw in the patter bl, br, tl, br, tr, tl and so, viewing the last two vertices as
            // ..bl, br], we need to point to bl (len - 2).
            let i = (vertices.len() - 2) as u16;
            let point = &points[points.len() - 1];
            let (left_point, right_point) =
                get_left_right_shifts(&point, &points[points.len() - 2]);
            vertices.extend([
                VertexInput::new(left_point.add(&point)),
                VertexInput::new(right_point.add(&point)),
            ]);
            indices.extend([i, i + 1, i + 2, i + 1, i + 3, i + 2]);
        }

        Self {
            vertices,
            indices,
            affine2: Affine2::default(),
        }
    }

    /// Return the bounding box that encapsulates this sprite
    pub fn bounds(&self) -> Bounds {
        let mut min = Vector2::new(f32::MAX, f32::MAX);
        let mut max = Vector2::new(f32::MIN, f32::MIN);

        for index in self.indices.iter() {
            let vertex = unsafe { self.vertices.get_unchecked(*index as usize) };
            if min.x > vertex.position.x {
                min.x = vertex.position.x;
            } else if max.x < vertex.position.x {
                max.x = vertex.position.x;
            }
            if min.y > vertex.position.y {
                min.y = vertex.position.y;
            } else if max.y < vertex.position.y {
                max.y = vertex.position.y;
            }
        }

        Bounds {
            bottom_left: min,
            top_right: max,
        }
    }

    /// ## TODO
    /// Update this method so that the various projection methods actually have the desired
    /// effect.
    pub(crate) fn update_texture_coordinates(&mut self, texture: &Texture) {
        match texture.projection_method {
            TextureProjectionMethod::ScaleToFit => {
                let bounds = self.bounds();
                let width = bounds.width();
                let height = bounds.height();
                //let center = bounds.center();

                let mesh_aspect_ratio = width / height;
                let texture_aspect_ratio = {
                    let (width, height) = texture.dimensions();
                    width as f32 / height as f32
                };

                let scale_factor_x = if mesh_aspect_ratio > texture_aspect_ratio {
                    // the mesh is wider than the texture, so we clip the sides
                    1.0 / height
                } else {
                    // the mesh is thinner than the texture, so we clip the top and
                    // bottom edges
                    1.0 / width
                };
                let scale_factor_y = -scale_factor_x;

                for vertex in self.vertices.iter_mut() {
                    vertex.texture_coordinates.x = vertex.position.x * scale_factor_x + 0.5;
                    vertex.texture_coordinates.y = vertex.position.y * scale_factor_y + 0.5;
                }
            }
            TextureProjectionMethod::SingleColor => {
                // all zero
            }
        }
    }

    pub(crate) fn index_buffer_contents(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(self.indices.as_ptr() as *const u8, self.indices.len() << 1)
        }
    }

    pub(crate) fn vertex_buffer_contents(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self.vertices.as_ptr() as *const u8,
                self.vertices.len() << 4,
            )
        }
    }
}
