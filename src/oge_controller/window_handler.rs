use crate::*;

#[derive(Clone, Copy, Default, Debug)]
/// Represents the size of a viewable window in pixels
pub struct WindowDimensions {
    pub width: u32,
    pub height: u32,
}

impl From<&winit::dpi::PhysicalSize<u32>> for WindowDimensions {
    fn from(physical_size: &winit::dpi::PhysicalSize<u32>) -> Self {
        Self {
            width: physical_size.width,
            height: physical_size.height,
        }
    }
}

pub struct WindowHandler {
    /// The dimensions of this window
    pub(crate) dimensions: WindowDimensions,
    /// The matrix that is used to transform points into normalized device coordinates
    pub(crate) matrix: crate::Matrix3x2,
}

impl WindowHandler{
    pub(crate) fn new(window: &winit::window::Window) -> Self {
        Self {
            dimensions: WindowDimensions::from(&window.inner_size()),
            matrix: Matrix3x2 {
                i: Vector2::new(1.0, 0.0),
                j: Vector2::new(0.0, 1.0),
                k: Vector2::new(0.0, 0.0)
            }
        }
    }

    pub(crate) fn set_viewable_region(&mut self, bounds: crate::Bounds) {
        let width = bounds.width();
        let height = bounds.height();
        let center = bounds.bottom_left + Vector2::new(width * 0.5, height * 0.5);

        self.matrix.i = Vector2::new(2.0 / width, 0.0);
        self.matrix.j = Vector2::new(0.0, 2.0 / height);
        self.matrix.k = Vector2::new(-center.x, -center.y);
    }
}



