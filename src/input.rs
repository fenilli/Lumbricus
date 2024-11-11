use std::collections::HashSet;

use winit::{event::KeyEvent, keyboard::KeyCode};

pub struct Input {
    key_downs: HashSet<KeyCode>,
    // key_ups: HashSet<KeyCode>,
}

impl Input {
    pub fn new() -> Self {
        Self {
            key_downs: HashSet::new(),
            // key_ups: HashSet::new(),
        }
    }

    pub fn clear(&mut self) {
        self.key_downs.clear();
        // self.key_ups.clear();
    }

    pub fn update(&mut self, event: &KeyEvent) {
        let key_code = match event.physical_key {
            winit::keyboard::PhysicalKey::Code(key) => key,
            _ => return,
        };

        match event.state {
            winit::event::ElementState::Pressed => {
                self.key_downs.insert(key_code);
            }
            _ => {} // winit::event::ElementState::Released => self.key_ups.insert(key_code),
        };
    }

    pub fn is_key_down(&self, key_code: KeyCode) -> bool {
        self.key_downs.contains(&key_code)
    }

    // pub fn is_key_ups(&self, key_code: KeyCode) -> bool {
    //     self.key_ups.contains(&key_code)
    // }
}
