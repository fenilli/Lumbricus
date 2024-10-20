use std::collections::HashSet;

use winit::{
    event::{ElementState, KeyEvent},
    keyboard::{KeyCode, PhysicalKey},
};

pub struct Input {
    held_keys: HashSet<KeyCode>,
    pressed_keys: HashSet<KeyCode>,
    released_keys: HashSet<KeyCode>,
}

impl Input {
    pub fn new() -> Self {
        Self {
            held_keys: HashSet::new(),
            pressed_keys: HashSet::new(),
            released_keys: HashSet::new(),
        }
    }

    pub fn clear(&mut self) {
        self.pressed_keys.clear();
        self.released_keys.clear();
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

    pub fn is_held(&self, key_code: KeyCode) -> bool {
        self.held_keys.contains(&key_code)
    }

    pub fn is_pressed(&self, key_code: KeyCode) -> bool {
        self.pressed_keys.contains(&key_code)
    }

    pub fn is_released(&self, key_code: KeyCode) -> bool {
        self.released_keys.contains(&key_code)
    }
}
