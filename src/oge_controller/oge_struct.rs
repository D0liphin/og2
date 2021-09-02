use crate::*;

pub struct Oge<'a, 'b: 'a> {
    pub(crate) handlers: &'a mut OgeHandlers,
    pub(crate) render_state: &'a mut RenderState,
    pub(crate) render_pass: RenderPass<'b>,
}

pub struct OgeHandlers {
    pub(crate) window_handler: WindowHandler,
    pub(crate) input_handler: InputHandler,
    pub(crate) meta_handler: MetaHandler,
}

impl<'a, 'b> Oge<'a, 'b> {
    pub fn create_texture(&self, config: &TextureConfiguration) -> Result<Texture, OgeError> {
        Texture::new(&self.render_state, &config)
    }

    pub fn draw_sprites<'c, T: IntoIterator<Item = &'c Sprite>>(&mut self, sprites: T) {
        for sprite in sprites {
            self.render_pass
                .render_bundles
                .push(sprite.get_render_bundle(&self));
        }
    }

    /// Sets the region of the coordinate system that should be displayed to the window.
    /// It is your responsibilty to ensure that this region has the same aspect ratio as the
    /// window.
    pub fn set_window_bounds(&mut self, bounds: Bounds) {
        self.handlers.window_handler.set_viewable_region(bounds);
    }

    /// Returns the dimensions of the window
    pub fn window_dimensions(&self) -> WindowDimensions {
        self.handlers.window_handler.dimensions
    }

    /// Returns `true` if the key with the key code `key_code` is pressed, and
    /// `false` if it is not. This is faster than using `Oge::get_key_status(&self, key_code: KeyCode)`
    pub fn get_key_down(&self, key_code: KeyCode) -> bool {
        self.handlers.input_handler.get_key_down(key_code)
    }

    /// Returns the `ButtonStatus` for a key with the specified `KeyCode`
    pub fn get_key_status(&self, key_code: KeyCode) -> ButtonStatus {
        self.handlers.input_handler.get_key_status(key_code)
    }

    /// Returns `true` if the mouse button with the given `MouseButtonCode` is pressed,
    /// and false if it s not
    pub fn get_mouse_button_down(&self, mouse_button_code: MouseButtonCode) -> bool {
        self.handlers
            .input_handler
            .get_mouse_button_down(mouse_button_code)
    }

    /// Returns the `ButtonStatus` for a mouse button with the specified `MouseButtonCode`.
    pub fn get_mouse_button_status(&self, mouse_button_code: MouseButtonCode) -> ButtonStatus {
        self.handlers
            .input_handler
            .get_mouse_button_status(mouse_button_code)
    }

    /// Returns the time, in seconds between the start of the previous update cycle
    /// and the start of the current update cycle.
    pub fn delta_time(&self) -> f32 {
        self.handlers.meta_handler.delta_time()
    }

    /// Returns the physical cursor position on the screen. Convert this
    /// To the coordinate system your window is using, with
    /// `Oge::get_real_cursor_position(&self, cursor_position: Vector2)`
    pub fn cursor_position(&self) -> Vector2 {
        self.handlers.input_handler.cursor_position()
    }

    /// Converts a physical position to the specified coordinate system
    pub fn get_real_position(&self, physical_position: Vector2) -> Vector2 {
        let real_position =
            physical_position.mul(&self.handlers.window_handler.reverse_affine2.matrix2);
        real_position.add(&self.handlers.window_handler.reverse_affine2.translation)
    }

    /// Returns a mutable reference to the component with the provided type
    pub fn get_component<T>(&mut self) -> &mut T {
        todo!()
    }
}

impl<'a, 'b> Oge<'a, 'b> {
    pub(crate) fn new(
        handlers: &'a mut OgeHandlers,
        render_state: &'a mut RenderState,
        render_pass_resources: &'b mut RenderPassResources,
    ) -> Self {
        let color_attachments = [wgpu::RenderPassColorAttachment {
            view: &render_pass_resources.surface_texture_view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    a: 1.0,
                }),
                store: true,
            },
        }];
        let render_pass =
            render_pass_resources
                .command_encoder
                .begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &color_attachments,
                    depth_stencil_attachment: None,
                });

        Self {
            handlers,
            render_state,
            render_pass: RenderPass {
                color_attachments,
                render_pass,
                render_bundles: &mut render_pass_resources.render_bundles,
            },
        }
    }

    pub(crate) fn resize(&mut self, window_dimensions: WindowDimensions) {
        self.handlers.window_handler.dimensions = window_dimensions;
    }

    pub(crate) fn finish(self) -> RenderPass<'b> {
        self.render_pass
    }
}
