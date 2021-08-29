use crate::*;

pub(crate) trait AsVector2 {
    fn from_vector2(vector2: Vector2) -> Self;

    fn as_vector2(&self) -> &Vector2 {
        unsafe { &*(self as *const Self as *const Vector2) }
    }
}

pub(crate) trait AsUsize {
    fn from_u16(index: u16) -> Self;

    fn as_usize(&self) -> usize;
}

macro_rules! index_from {
    ($input:expr) => {
        Self::IndexRepr::from_u16($input)
    };
}

macro_rules! vertex_from {
    ($input:expr) => {
        Self::VertexRepr::from_vector2($input)
    };
}

pub(crate) trait Mesh: Sized {
    type VertexRepr: AsVector2 + Copy;
    type IndexRepr: AsUsize + Copy;

    fn from_parts(vertices: Vec<Self::VertexRepr>, indices: Vec<Self::IndexRepr>) -> Self;

    fn vertices(&self) -> &Vec<Self::VertexRepr>;

    fn indices(&self) -> &Vec<Self::IndexRepr>;

    fn new_rectangle(width: f32, height: f32) -> Self {
        let (x, y) = (width / 2.0, height / 2.0);

        Self::from_parts(
            vec![
                vertex_from!(Vector2::new(-x, -y)),
                vertex_from!(Vector2::new(x, -y)),
                vertex_from!(Vector2::new(x, y)),
                vertex_from!(Vector2::new(-x, y)),
            ],
            vec![
                index_from!(0),
                index_from!(1),
                index_from!(2),
                index_from!(0),
                index_from!(2),
                index_from!(3),
            ],
        )
    }

    /// Returns the bounds of this mesh
    fn bounds(&self) -> Bounds {
        let indices = self.indices();
        let vertices = self.vertices();

        let mut min = Vector2::new(f32::MAX, f32::MAX);
        let mut max = Vector2::new(f32::MIN, f32::MIN);

        for index in indices.iter() {
            let vertex = unsafe { vertices.get_unchecked(index.as_usize()).as_vector2() };
            if min.x > vertex.x {
                min.x = vertex.x;
            } else if max.x < vertex.x {
                max.x = vertex.x;
            }
            if min.y > vertex.y {
                min.y = vertex.y;
            } else if max.y < vertex.y {
                max.y = vertex.y;
            }
        }

        Bounds {
            bottom_left: min,
            top_right: max,
        }
    }
}
