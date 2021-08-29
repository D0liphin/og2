use crate::*;

#[derive(Clone, Copy, Default)]
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

    /// Sets the region of the coordinate system that should be displayed to the window.
    /// It is your responsibilty to ensure that this region has the same aspect ratio as the
    /// window.
    pub fn set_viewable_region(&mut self, bounds: crate::Bounds) {
        todo!()
    }
}



