use crate::*;
use std::time;

struct InputState {
    state: Vec<ButtonStatus>,
}

impl InputState {
    fn new(capacity: usize) -> Self {
        let mut state = Vec::with_capacity(256);
        for _ in 0..256 {
            state.push(ButtonStatus::Released);
        }
        Self { state }
    }

    fn set(&mut self, index: usize, set_value: bool) {
        let button_status = self.state.get_mut(index).unwrap();
        if set_value {
            match button_status {
                ButtonStatus::Pressed(ref mut instant) => *instant = time::Instant::now(),
                ButtonStatus::Released => *button_status = ButtonStatus::Pressed(time::Instant::now()),
            }
        } else {
            match button_status {
                ButtonStatus::Pressed(_) => 
                _ => {}
            }
        }
    }
}

pub enum ButtonStatus {
    Pressed(time::Instant),
    Released,
}

impl ButtonStatus {
    /// Returns the approximate time this key has been pressed in seconds as an `f32`.   
    /// If this key is not pressed, this function returns `None`.   
    /// ## Example
    /// ```rs
    /// if let oge.get_key_status(oge::KeyCode::Space).time_pressed_f32() = Some(time_pressed) {
    ///     if time_pressed < oge.delta_time() {
    ///         // Space bar was just pressed
    ///     }
    /// }
    /// ```
    pub fn time_pressed_f32(&self) -> Option<f32> {
        match self {
            ButtonStatus::Pressed(start_instant) => Some(
                time::Instant::now()
                    .duration_since(*start_instant)
                    .as_micros() as f32
                    / 1_000_000.,
            ),
            ButtonStatus::Released => None,
        }
    }
}

pub(crate) struct InputHandler {
    keyboard_input_state: InputState,
    mouse_input_state: InputState,
    cursor_physical_position: Vector2,
}

impl InputHandler {
    pub(crate) fn new() -> Self {
        Self {
            keyboard_input_state: InputState::new(256),
            mouse_input_state: InputState::new(256),
            cursor_physical_position: Vector2::ZERO,
        }
    }

    pub(crate) fn set_keyboard_input_state(&mut self, key_code: KeyCode, set: bool) {
        todo!()
    }

    pub(crate) fn get_key_status(&self, key_code: KeyCode) -> ButtonStatus {
        todo!()
    }

    pub(crate) fn get_key_down(&self, key_code: KeyCode) -> bool {
        todo!()
    }

    pub(crate) fn set_mouse_input_state(&mut self, mouse_button_code: MouseButtonCode, set: bool) {
        todo!()
    }

    pub(crate) fn get_mouse_button_status(
        &self,
        mouse_button_code: MouseButtonCode,
    ) -> ButtonStatus {
        todo!()
    }

    pub(crate) fn get_mouse_button_down(&self, mouse_button_code: MouseButtonCode) -> bool {
        todo!()
    }

    pub(crate) fn set_cursor_physical_position(
        &mut self,
        physical_position: &winit::dpi::PhysicalPosition<f64>,
    ) {
        self.cursor_physical_position =
            Vector2::new(physical_position.x as f32, physical_position.y as f32);
    }

    pub(crate) fn cursor_position(&self) -> Vector2 {
        self.cursor_physical_position
    }
}
