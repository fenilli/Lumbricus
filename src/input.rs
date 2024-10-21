use std::collections::HashSet;

use winit::{
    event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta},
    keyboard::{KeyCode, PhysicalKey},
};

pub struct Input {
    // Keyboard tracking
    held_keys: HashSet<KeyCode>,
    pressed_keys: HashSet<KeyCode>,
    released_keys: HashSet<KeyCode>,

    // Mouse tracking
    held_buttons: HashSet<MouseButton>,
    pressed_buttons: HashSet<MouseButton>,
    released_buttons: HashSet<MouseButton>,
    mouse_position: (f64, f64),
    scroll_delta: (f32, f32),
}

impl Input {
    pub fn new() -> Self {
        Self {
            held_keys: HashSet::new(),
            pressed_keys: HashSet::new(),
            released_keys: HashSet::new(),

            held_buttons: HashSet::new(),
            pressed_buttons: HashSet::new(),
            released_buttons: HashSet::new(),
            mouse_position: (0.0, 0.0),
            scroll_delta: (0.0, 0.0),
        }
    }

    pub fn clear(&mut self) {
        self.pressed_keys.clear();
        self.released_keys.clear();

        self.pressed_buttons.clear();
        self.released_buttons.clear();
        self.scroll_delta = (0.0, 0.0);
    }

    pub fn handle_keyboard_event(&mut self, event: KeyEvent) {
        let state = event.state;
        let key_code = match event.physical_key {
            PhysicalKey::Code(code) => code,
            PhysicalKey::Unidentified(_) => return,
        };

        match state {
            ElementState::Pressed => {
                if !self.held_keys.contains(&key_code) {
                    self.pressed_keys.insert(key_code);
                }

                self.held_keys.insert(key_code);
            }
            ElementState::Released => {
                self.held_keys.remove(&key_code);
                self.released_keys.insert(key_code);
            }
        }
    }

    pub fn handle_mouse_button(&mut self, state: ElementState, button: MouseButton) {
        match state {
            ElementState::Pressed => {
                if !self.held_buttons.contains(&button) {
                    self.pressed_buttons.insert(button);
                }

                self.held_buttons.insert(button);
            }
            ElementState::Released => {
                self.held_buttons.remove(&button);
                self.released_buttons.insert(button);
            }
        }
    }

    pub fn handle_cursor_moved(&mut self, position: (f64, f64)) {
        self.mouse_position = position;
    }

    pub fn handle_scroll_delta(&mut self, delta: MouseScrollDelta) {
        match delta {
            MouseScrollDelta::LineDelta(x, y) => {
                self.scroll_delta.0 += x;
                self.scroll_delta.1 += y;
            }
            MouseScrollDelta::PixelDelta(pos) => {
                self.scroll_delta.0 += pos.x as f32;
                self.scroll_delta.1 += pos.y as f32;
            }
        }
    }

    pub fn is_key_held(&self, key_code: KeyCode) -> bool {
        self.held_keys.contains(&key_code)
    }

    pub fn is_key_pressed(&self, key_code: KeyCode) -> bool {
        self.pressed_keys.contains(&key_code)
    }

    pub fn is_key_released(&self, key_code: KeyCode) -> bool {
        self.released_keys.contains(&key_code)
    }

    pub fn is_button_held(&self, button: MouseButton) -> bool {
        self.held_buttons.contains(&button)
    }

    pub fn is_button_pressed(&self, button: MouseButton) -> bool {
        self.pressed_buttons.contains(&button)
    }

    pub fn is_button_released(&self, button: MouseButton) -> bool {
        self.released_buttons.contains(&button)
    }

    pub fn get_mouse_position(&self) -> (f64, f64) {
        self.mouse_position
    }

    pub fn get_scroll_delta(&self) -> (f32, f32) {
        self.scroll_delta
    }
}
