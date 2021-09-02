use crate::*;
use std::collections::HashSet;

#[derive(Debug)]
struct InputState {
    state: Vec<ButtonStatus>,
    updated_button_indices: HashSet<usize>,
}

impl InputState {
    fn new(capacity: usize) -> Self {
        let mut state = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            state.push(ButtonStatus {
                button_state: ButtonState::Released,
                pressed_count: 0,
                released_count: 0,
            });
        }
        Self {
            state,
            updated_button_indices: HashSet::new(),
        }
    }

    fn set(&mut self, index: usize, set_value: ButtonState) {
        self.updated_button_indices.insert(index);
        let button_status = self.state.get_mut(index).unwrap();
        match set_value {
            ButtonState::Pressed => {
                button_status.pressed_count += 1;
                button_status.button_state = ButtonState::Pressed;
            }
            ButtonState::Released => {
                button_status.released_count += 1;
                button_status.button_state = ButtonState::Released;
            }
        }
    }

    fn get(&self, index: usize) -> ButtonStatus {
        self.state[index]
    }

    fn update(&mut self) {
        for i in self.updated_button_indices.drain() {
            let button_status = self.state.get_mut(i).unwrap();
            button_status.pressed_count = 0;
            button_status.released_count = 0;
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ButtonState {
    /// The button's last known state is pressed
    Pressed = 0,
    /// The button's last known state is released
    Released = 1,
}

/// Describes the current state of a button. Button states are kept track of seperately,
/// and may change from pressed to released and vice versa several times between update
/// cycles.
#[derive(Debug, Clone, Copy)]
pub struct ButtonStatus {
    /// The current `ButtonState` of this button.
    pub button_state: ButtonState,
    /// The number of times the button was pressed since the last update cycle
    pub pressed_count: u32,
    /// The number of times the button was released since the last update cycle
    pub released_count: u32,
}

impl ButtonStatus {}

pub(crate) struct InputHandler {
    keyboard_input_state: InputState,
    mouse_input_state: InputState,
    cursor_physical_position: Vector2,
}

impl InputHandler {
    pub(crate) fn new() -> Self {
        Self {
            keyboard_input_state: InputState::new(256),
            mouse_input_state: InputState::new(8),
            cursor_physical_position: Vector2::ZERO,
        }
    }

    pub(crate) fn set_keyboard_input_state(&mut self, key_code: usize, set: ButtonState) {
        self.keyboard_input_state.set(key_code, set);
    }

    pub(crate) fn get_key_status(&self, key_code: KeyCode) -> ButtonStatus {
        self.keyboard_input_state.get(key_code as usize)
    }

    pub(crate) fn get_key_down(&self, key_code: KeyCode) -> bool {
        self.keyboard_input_state
            .get(key_code as usize)
            .button_state
            == ButtonState::Pressed
    }

    pub(crate) fn set_mouse_input_state(&mut self, mouse_button_code: usize, set: ButtonState) {
        self.mouse_input_state.set(mouse_button_code, set);
    }

    pub(crate) fn get_mouse_button_status(
        &self,
        mouse_button_code: MouseButtonCode,
    ) -> ButtonStatus {
        self.mouse_input_state.get(mouse_button_code as usize)
    }

    pub(crate) fn get_mouse_button_down(&self, mouse_button_code: MouseButtonCode) -> bool {
        self.mouse_input_state
            .get(mouse_button_code as usize)
            .button_state
            == ButtonState::Pressed
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

    pub(crate) fn update(&mut self) {
        self.keyboard_input_state.update();
        self.mouse_input_state.update();
    }
}
