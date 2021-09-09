use crate::*;

pub struct Oge<'a, 'b: 'a> {
    pub(crate) handlers: &'a mut OgeHandlers,
    pub(crate) render_state: &'a mut RenderState,
    pub(crate) render_pass: RenderPass<'b>,
    pub(crate) queued_operations: Vec<Operation>,
}

pub struct OgeHandlers {
    pub(crate) window_handler: WindowHandler,
    pub(crate) input_handler: InputHandler,
    pub(crate) meta_handler: MetaHandler,
}

impl OgeHandlers {
    pub(crate) fn new(window: &winit::window::Window) -> Self {
        Self {
            window_handler: WindowHandler::new(&window),
            input_handler: InputHandler::new(),
            meta_handler: MetaHandler::new(),
        }
    }
}

impl<'a, 'b> Oge<'a, 'b> {
    /// Create a new `Sprite`
    pub fn create_sprite(&self, config: SpriteConfiguration) -> Result<Sprite> {
        Sprite::new(config)
    }

    /// Create a new `Texture`
    pub fn create_texture(&self, config: &TextureConfiguration) -> Result<Texture> {
        Texture::new(&self.render_state, config)
    }

    /// Draws a single render bundle
    pub fn draw_once(&mut self, render_bundle: impl IntoRenderBundle) {
        self.render_pass
            .render_bundles
            .push(render_bundle.get_render_bundle(&self))
    }

    /// Draws several render bundles.
    pub fn draw<T: IntoRenderBundle>(&mut self, render_bundles: impl IntoIterator<Item = T>) {
        for render_bundle in render_bundles {
            self.draw_once(render_bundle.get_render_bundle(&self));
        }
    }

    fn destructure_color_width_opacity(
        &self,
        color_width_opacity: Option<(Option<Color>, Option<f32>, Option<f32>)>,
    ) -> (Color, f32, f32) {
        let (color, width, opacity) = color_width_opacity.unwrap_or((None, None, None));
        let color = color.unwrap_or(Color::RED);
        let width = width.unwrap_or(5.);
        let opacity = opacity.unwrap_or(0.8);
        (color, width, opacity)
    }

    /// Draws a line for debugging. Do not use this for actual line drawing - make a curve and
    /// modify its points instead.
    ///
    /// `color_width_opacity` defaults to `(Color::RED, 5., 0.8)`
    pub fn draw_debug_line(
        &mut self,
        points: Vec<Vector2>,
        color_width_opacity: Option<(Option<Color>, Option<f32>, Option<f32>)>,
    ) {
        let (color, width, opacity) = self.destructure_color_width_opacity(color_width_opacity);
        let default_texture =
            if let Ok(texture) = self.create_texture(&TextureConfiguration::color(color)) {
                texture
            } else {
                return;
            };
        let mut curve = if let Ok(curve) = sprite::Curve::new(sprite::CurveConfiguration {
            label: Some("Debug Line"),
            points,
            width,
            opacity,
            style: CurveStyle::DoubleJointed,
            z_index: ZIndex::AboveAll,
            default_texture,
            is_loop: false,
            texture_projection_method: TextureProjectionMethod::SingleColor,
        }) {
            curve
        } else {
            return;
        };

        self.draw_once(curve.get_sprite());
    }

    /// Same effect as `Oge::draw_debug_line`, but adds an arrow tip to the end of the line.
    pub fn draw_debug_arrow(
        &mut self,
        points: Vec<Vector2>,
        color_width_opacity: Option<(Option<Color>, Option<f32>, Option<f32>)>,
    ) {
        let arrow_head_position = {
            let last_point = points[points.len() - 1];
            let penultimate_point = points[points.len() - 2];
            last_point.sub(&penultimate_point).with_magnitude(last_point.distance_to(&penultimate_point) - )
        };
        self.draw_debug_line(points, color_width_opacity);
        
        let (color, width, opacity) = self.destructure_color_width_opacity(color_width_opacity);
        let default_texture =
            if let Ok(texture) = self.create_texture(&TextureConfiguration::color(color)) {
                texture
            } else {
                return;
            }; 

        let mut arrow_head_sprite = if let Ok(sprite) = self.create_sprite(SpriteConfiguration {
            label: Some("Debug Arrow Head"),
            mesh: SpriteMesh::new_elipse(width * 2., width * 2., 3),
            z_index: ZIndex::AboveAll,
            default_texture,
            opacity,
            texture_projection_method: TextureProjectionMethod::SingleColor,
        }) {
            sprite
        } else {
            return;
        };
        arrow_head_sprite.set_position(arrow_head_position);
        self.draw_once(&arrow_head_sprite);
    }

    /// Configures the render pipeline used
    pub fn configure_render_pipeline(&mut self, config: RenderPipelineConfiguration) {
        self.queued_operations
            .push(Operation::UpdateRenderPipelineConfiguration(config));
    }

    /// Sets the region of the coordinate system that should be displayed to the window.
    /// It is your responsibilty to ensure that this region has the same aspect ratio as the
    /// window.
    pub fn set_window_bounds(&mut self, bounds: Bounds) {
        self.handlers.window_handler.set_viewable_region(bounds);
    }

    /// Returns the last set `Bounds` for the window
    pub fn window_bounds(&self) -> Bounds {
        self.handlers.window_handler.bounds
    }

    /// Returns `true` if the window has been resized and this resize has not yet been handled.
    /// Calling this function indicates that you will handle the window resize
    pub fn window_has_resized(&mut self) -> bool {
        self.handlers.window_handler.resized()
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
    pub fn get_real_position(&self, physical_position: &Vector2) -> Vector2 {
        let real_position =
            physical_position.mul(&self.handlers.window_handler.reverse_affine2.matrix2);
        real_position.add(&self.handlers.window_handler.reverse_affine2.translation)
    }

    /// Gets the most recent mouse position, with its coordinates converted to the specified
    /// coordinate system
    pub fn get_real_cursor_position(&self) -> Vector2 {
        self.get_real_position(&self.cursor_position())
    }

    /// Returns a `Vec` containing all the cursor positions since the last update. Often,
    /// the cursor will have its position updated several times in-between frames, so use
    /// this to get the most accurate path of the cursor
    pub fn cursor_positions(&self) -> Vec<Vector2> {
        self.handlers.input_handler.cursor_positions()
    }

    /// Gets the physical cursor positions (see `oge.cursor_positions()`) and applies
    /// get_real_position
    pub fn get_real_cursor_positions(&self) -> Vec<Vector2> {
        self.cursor_positions()
            .into_iter()
            .map(|physical_position| self.get_real_position(&physical_position))
            .collect()
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
        let (view, resolve_target) = if render_state.sample_count > 1 {
            (
                &render_pass_resources.multisampled_frame_buffer_view,
                Some(&render_pass_resources.surface_texture_view),
            )
        } else {
            (&render_pass_resources.surface_texture_view, None)
        };

        let _color_attachments = [wgpu::RenderPassColorAttachment {
            view,
            resolve_target,
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
                    color_attachments: &_color_attachments,
                    depth_stencil_attachment: None,
                });

        Self {
            handlers,
            render_state,
            render_pass: RenderPass {
                _color_attachments,
                render_pass,
                render_bundles: &mut render_pass_resources.render_bundles,
            },
            queued_operations: vec![],
        }
    }

    /// Consume this controller and return its `RenderPass` for drawing.
    pub(crate) fn finish(self) -> (RenderPass<'b>, Vec<Operation>) {
        (self.render_pass, self.queued_operations)
    }
}
