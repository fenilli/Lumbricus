use std::collections::HashMap;

use winit::{
    event::{ElementState, KeyEvent},
    keyboard::{KeyCode, PhysicalKey},
};

pub struct Input {
    keys: HashMap<KeyCode, ElementState>,
}

impl Input {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.keys.clear();
    }

    pub fn update(&mut self, event: &KeyEvent) {
        let code = if let PhysicalKey::Code(code) = event.physical_key {
            Some((code, event.state))
        } else {
            None
        };

        if let Some((code, state)) = code {
            self.keys.insert(code, state);
        }
    }

    pub fn is_key_pressed(&self, key_code: KeyCode) -> bool {
        if let Some((code, state)) = self.keys.get_key_value(&key_code) {
            state.is_pressed() && *code == key_code
        } else {
            false
        }
    }

    pub fn is_key_released(&self, key_code: KeyCode) -> bool {
        if let Some((code, state)) = self.keys.get_key_value(&key_code) {
            !state.is_pressed() && *code == key_code
        } else {
            false
        }
    }
}
