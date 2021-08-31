use crate::*;

pub struct Oge {
    pub(crate) window_handler: WindowHandler,
    pub(crate) render_state: RenderState,
    pub(crate) input_handler: InputHandler,
    pub(crate) meta_handler: MetaHandler,
}

impl Oge {
    pub fn create_texture(&self, config: &TextureConfiguration) -> Result<Texture, OgeError> {
        Texture::new(&self.render_state, &config)
    }

    pub fn render_sprites<'a, T: IntoIterator<Item = &'a Sprite>>(&self, sprites: T) {
        let surface_texture = match self.render_state.surface.get_current_frame() {
            Err(e) => return,
            Ok(frame) => frame,
        }
        .output;

        {
            let surface_texture_view = surface_texture
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());

            for sprite in sprites {
                let mut render_bundle = sprite.get_render_bundle();

                let mut render_bundle_affine2 = render_bundle.affine2.clone();
                render_bundle_affine2
                    .translation
                    .mul_assign(&self.window_handler.affine2.matrix2);
                render_bundle_affine2 =
                    render_bundle_affine2.reverse_compose(&self.window_handler.affine2);

                render_bundle.affine2 = &render_bundle_affine2;
                self.render_state
                    .render(&surface_texture_view, render_bundle);
            }
        }
        // done!
    }

    /// Sets the region of the coordinate system that should be displayed to the window.
    /// It is your responsibilty to ensure that this region has the same aspect ratio as the
    /// window.
    pub fn set_window_bounds(&mut self, bounds: Bounds) {
        self.window_handler.set_viewable_region(bounds);
    }

    /// Returns the dimensions of the window
    pub fn window_dimensions(&self) -> WindowDimensions {
        self.window_handler.dimensions
    }

    /// Returns the `ButtonStatus` of the key with this key code.
    pub fn get_key_status(&self, key_code: KeyCode) -> ButtonStatus {
        self.input_handler.get_key_status(key_code as u8)
    }

    /// Returns `true` if the key with the key code `key_code` is pressed, and
    /// `false` if it is not. This is faster than using `Oge::get_key_status(&self, key_code: KeyCode)`
    pub fn get_key_down(&self, key_code: KeyCode) -> bool {
        self.input_handler.get_key_down(key_code as u8)
    }

    /// Returns the `ButtonStatus` of the mouse button with the provided `MouseButtonCode`
    pub fn get_mouse_button_status(&self, mouse_button_code: MouseButtonCode) -> ButtonStatus {
        self.input_handler
            .get_mouse_button_status(mouse_button_code as u8)
    }

    /// Returns `true` if the mouse button with the given `MouseButtonCode` is pressed,
    /// and false if it s not
    pub fn get_mouse_button_down(&self, mouse_button_code: MouseButtonCode) -> bool {
        self.input_handler
            .get_mouse_button_down(mouse_button_code as u8)
    }

    /// Returns the time, in seconds between the start of the previous update cycle
    /// and the start of the current update cycle.
    pub fn delta_time(&self) -> f32 {
        self.meta_handler.delta_time()
    }

    /// Returns the physical cursor position on the screen. Convert this
    /// To the coordinate system your window is using, with
    /// `Oge::get_real_cursor_position(&self, cursor_position: Vector2)`
    pub fn cursor_position(&self) -> Vector2 {
        self.input_handler.cursor_position()
    }

    /// Converts a physical position to the specified coordinate system
    pub fn get_real_position(&self, physical_position: Vector2) -> Vector2 {
        let real_position = physical_position.mul(&self.window_handler.reverse_affine2.matrix2);
        real_position.add(&self.window_handler.reverse_affine2.translation)
    }

    /// Returns a mutable reference to the component with the provided type
    pub fn get_component<T>(&mut self) -> &mut T {
        todo!()
    }
}

impl Oge {
    pub(crate) fn resize(&mut self, window_dimensions: WindowDimensions) {
        self.render_state.resize(&window_dimensions);
        self.window_handler.dimensions = window_dimensions;
    }
}
