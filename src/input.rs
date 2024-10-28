use std::collections::{HashMap, HashSet};

use winit::{
    event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent},
    keyboard::{KeyCode, ModifiersState, PhysicalKey},
};

pub struct BindingDescriptor {
    pub keys: HashSet<KeyCode>,
    pub buttons: HashSet<MouseButton>,
    pub modifiers: ModifiersState,
}

impl Default for BindingDescriptor {
    fn default() -> Self {
        Self {
            keys: HashSet::new(),
            buttons: HashSet::new(),
            modifiers: ModifiersState::empty(),
        }
    }
}

pub struct Input {
    // Keyboard tracking
    held_keys: HashSet<KeyCode>,
    pressed_keys: HashSet<KeyCode>,
    released_keys: HashSet<KeyCode>,
    modifiers: ModifiersState,

    // Mouse tracking
    held_buttons: HashSet<MouseButton>,
    pressed_buttons: HashSet<MouseButton>,
    released_buttons: HashSet<MouseButton>,
    mouse_position: (f64, f64),
    scroll_delta: (f32, f32),

    bindings: HashMap<&'static str, BindingDescriptor>,
}

#[allow(dead_code)]
impl Input {
    pub fn new() -> Self {
        Self {
            held_keys: HashSet::new(),
            pressed_keys: HashSet::new(),
            released_keys: HashSet::new(),
            modifiers: ModifiersState::empty(),

            held_buttons: HashSet::new(),
            pressed_buttons: HashSet::new(),
            released_buttons: HashSet::new(),
            mouse_position: (0.0, 0.0),
            scroll_delta: (0.0, 0.0),

            bindings: HashMap::new(),
        }
    }

    pub fn bind_action(&mut self, action: &'static str, binding: BindingDescriptor) {
        self.bindings.insert(action, binding);
    }

    pub fn clear_bindings(&mut self) {
        self.bindings.clear();
    }

    pub fn clear(&mut self) {
        self.pressed_keys.clear();
        self.released_keys.clear();

        self.pressed_buttons.clear();
        self.released_buttons.clear();
        self.scroll_delta = (0.0, 0.0);
    }

    pub fn update(&mut self, event: &WindowEvent) {
        self.handle_keyboard_event(event);
        self.handle_mouse_event(event);
    }

    fn handle_keyboard_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                let element_state = event.state;
                let key_code = match event.physical_key {
                    PhysicalKey::Code(code) => code,
                    PhysicalKey::Unidentified(_) => return,
                };

                match element_state {
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
            WindowEvent::ModifiersChanged(modifiers) => {
                self.modifiers = modifiers.state();
            }
            _ => {}
        };
    }

    fn handle_mouse_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::MouseInput { state, button, .. } => match state {
                ElementState::Pressed => {
                    if !self.held_buttons.contains(&button) {
                        self.pressed_buttons.insert(*button);
                    }

                    self.held_buttons.insert(*button);
                }
                ElementState::Released => {
                    self.held_buttons.remove(&button);
                    self.released_buttons.insert(*button);
                }
            },
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_position = (position.x, position.y);
            }
            WindowEvent::MouseWheel { delta, .. } => match delta {
                MouseScrollDelta::LineDelta(x, y) => {
                    self.scroll_delta.0 += x;
                    self.scroll_delta.1 += y;
                }
                MouseScrollDelta::PixelDelta(pos) => {
                    self.scroll_delta.0 += pos.x as f32;
                    self.scroll_delta.1 += pos.y as f32;
                }
            },
            _ => {}
        };
    }

    pub fn is_action_held(&self, action: &'static str) -> bool {
        if let Some(binding) = self.bindings.get(&action) {
            let keys_match = binding.keys.is_subset(&self.held_keys);
            let buttons_match = binding.buttons.is_subset(&self.held_buttons);
            let modifiers_match = binding.modifiers == self.modifiers;
            keys_match && buttons_match && modifiers_match
        } else {
            false
        }
    }

    pub fn is_action_pressed(&self, action: &'static str) -> bool {
        if let Some(binding) = self.bindings.get(&action) {
            let keys_match = binding.keys.is_subset(&self.pressed_keys);
            let buttons_match = binding.buttons.is_subset(&self.pressed_buttons);
            let modifiers_match = binding.modifiers == self.modifiers;
            keys_match && buttons_match && modifiers_match
        } else {
            false
        }
    }

    pub fn is_action_released(&self, action: &'static str) -> bool {
        if let Some(binding) = self.bindings.get(&action) {
            let keys_match = binding.keys.is_subset(&self.released_keys);
            let buttons_match = binding.buttons.is_subset(&self.released_buttons);
            let modifiers_match = binding.modifiers == self.modifiers;
            keys_match && buttons_match && modifiers_match
        } else {
            false
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
