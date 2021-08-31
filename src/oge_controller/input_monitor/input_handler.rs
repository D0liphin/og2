use crate::*;
use cgmath::Zero;

#[derive(Default)]
pub(crate) struct InputHandler {
    old_keyboard_input_state: [u64; 4],
    keyboard_input_state: [u64; 4],
    old_mouse_input_state: u64,
    mouse_input_state: u64,
    cursor_physical_position: Vector2,
}

#[repr(u8)]
pub enum ButtonStatus {
    // The Input is not being pressed
    Released = 0b00,
    // The Input was just pressed (in the time betwen the last update
    // and this update)
    JustPressed = 0b01,
    // The Input was just released (in the time betwen the last update
    // and this update)
    JustReleased = 0b10,
    // The Input is being pressed
    Pressed = 0b11,
}

impl ButtonStatus {
    fn new(l: bool, r: bool) -> Self {
        match (l, r) {
            (false, false) => ButtonStatus::Released,
            (false, true) => ButtonStatus::JustPressed,
            (true, false) => ButtonStatus::JustReleased,
            (true, true) => ButtonStatus::Pressed,
        }
    }
}

impl InputHandler {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    fn get_key_code_chunk(key_code: u8) -> usize {
        key_code as usize >> 6
    }

    fn get_key_code_mask(key_code: u8) -> u64 {
        1_u64 << (key_code & 0b11_1111)
    }

    pub(crate) fn set_keyboard_input_state(&mut self, key_code: u8, set: bool) {
        unsafe {
            let chunk = Self::get_key_code_chunk(key_code);
            let mut new_chunk = self.keyboard_input_state.get_unchecked_mut(chunk);
            *self.old_keyboard_input_state.get_unchecked_mut(chunk) = *new_chunk;

            let mask = Self::get_key_code_mask(key_code);
            *new_chunk = if set {
                *new_chunk | mask
            } else {
                *new_chunk & !mask
            };
        }
    }

    pub(crate) fn get_key_status(&self, key_code: u8) -> ButtonStatus {
        let chunk = Self::get_key_code_chunk(key_code);
        let mask = Self::get_key_code_mask(key_code);

        ButtonStatus::new(
            unsafe { self.old_keyboard_input_state.get_unchecked(chunk) & mask > 0 },
            unsafe { self.keyboard_input_state.get_unchecked(chunk) & mask > 0 },
        )
    }

    pub(crate) fn get_key_down(&self, key_code: u8) -> bool {
        let chunk = Self::get_key_code_chunk(key_code);
        let mask = Self::get_key_code_mask(key_code);
        unsafe { self.keyboard_input_state.get_unchecked(chunk) & mask > 0 }
    }

    pub(crate) fn set_mouse_input_state(&mut self, mouse_button_code: u8, set: bool) {
        self.old_mouse_input_state = self.mouse_input_state;
        let mask = 1 << mouse_button_code;
        self.mouse_input_state = if set {
            self.mouse_input_state | mask
        } else {
            self.mouse_input_state & !mask
        };
    }

    pub(crate) fn get_mouse_button_status(&self, mouse_button_code: u8) -> ButtonStatus {
        let mask = 1 << mouse_button_code;
        ButtonStatus::new(unsafe { self.old_mouse_input_state & mask > 0 }, unsafe {
            self.mouse_input_state & mask > 0
        })
    }

    pub(crate) fn get_mouse_button_down(&self, mouse_button_code: u8) -> bool {
        self.mouse_input_state & (1 << mouse_button_code) > 0
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
