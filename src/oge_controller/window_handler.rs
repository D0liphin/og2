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
    /// The affine matrix that is used to transform points into normalized device coordinates
    pub(crate) affine2: Affine2,
    /// Affine matrix used to convert a physical position to a viewable region point
    pub(crate) reverse_affine2: Affine2,
    /// `true` if the window dimensions have been updated since the last update cycle.
    pub(crate) dimensions_updated: bool,
}

impl WindowHandler {
    pub(crate) fn new(window: &winit::window::Window) -> Self {
        Self {
            dimensions: WindowDimensions::from(&window.inner_size()),
            affine2: Affine2::default(),
            reverse_affine2: Affine2::default(),
            dimensions_updated: true,
        }
    }

    pub(crate) fn set_viewable_region(&mut self, bounds: crate::Bounds) {
        let width = bounds.width();
        let height = bounds.height();
        let frac_width_2 = width * 0.5;
        let frac_height_2 = height * 0.5;
        let center = bounds
            .bottom_left
            .add(&Vector2::new(frac_width_2, frac_height_2));

        self.affine2 = Affine2 {
            matrix2: Matrix2 {
                i: Vector2::new(2.0 / width, 0.0),
                j: Vector2::new(0.0, 2.0 / height),
            },
            translation: Vector2::new(-center.x, -center.y),
        };

        let (window_width, window_height) =
            (self.dimensions.width as f32, self.dimensions.height as f32);
        self.reverse_affine2 = Affine2 {
            matrix2: Matrix2 {
                i: Vector2::new(width / window_width, 0.0),
                j: Vector2::new(0.0, -height / window_height),
            },
            translation: Vector2::new(-frac_width_2, frac_height_2),
        }
    }

    pub(crate) fn resize(&mut self, dimensions: WindowDimensions) {
        self.dimensions = dimensions;
        self.dimensions_updated = true;
    }

    pub(crate) fn resized (&mut self) -> bool {
        let res = self.dimensions_updated;
        self.dimensions_updated = false;
        res
    }
}
