#[macro_export]
macro_rules! load_window_handler {
    ($pixel_width:expr, $pixel_height:expr) => {{
        struct WindowHandler;

        impl Script for WindowHandler {
            fn start(_: &mut crate::Oge) -> Self {
                Self
            }

            fn update(&mut self, oge: &mut crate::Oge) {
                if oge.window_has_resized() {
                    let window_dimensions = oge.window_dimensions();
                    let (x, y) = (
                        window_dimensions.width as f32 * 0.5 * $pixel_width,
                        window_dimensions.height as f32 * 0.5 * $pixel_height,
                    );
                    oge.set_window_bounds(oge::Bounds {
                        bottom_left: oge::Vector2::new(-x, -y),
                        top_right: oge::Vector2::new(x, y),
                    })
                }
            }
        }

        WindowHandler::load_script()
    }};
}
