use std::collections::HashSet;

use winit::{
    event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent},
    keyboard::{KeyCode, ModifiersState, PhysicalKey},
};

pub struct Input {
    // Keyboard tracking
    keys_pressed: HashSet<KeyCode>,
    keys_released: HashSet<KeyCode>,
    modifiers: ModifiersState,

    // Mouse tracking
    buttons_pressed: HashSet<MouseButton>,
    buttons_released: HashSet<MouseButton>,
    mouse_position: (f64, f64),
    scroll_delta: (f32, f32),
}

#[allow(dead_code)]
impl Input {
    pub fn new() -> Self {
        Self {
            keys_pressed: HashSet::new(),
            keys_released: HashSet::new(),
            modifiers: ModifiersState::empty(),

            buttons_pressed: HashSet::new(),
            buttons_released: HashSet::new(),
            mouse_position: (0.0, 0.0),
            scroll_delta: (0.0, 0.0),
        }
    }

    pub fn clear(&mut self) {
        self.keys_pressed.clear();
        self.keys_released.clear();
        self.buttons_pressed.clear();
        self.buttons_released.clear();
        self.scroll_delta = (0.0, 0.0);
    }

    pub fn update(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                match event.physical_key {
                    PhysicalKey::Code(code) => {
                        match event.state {
                            ElementState::Pressed => {
                                self.keys_pressed.insert(code);
                            }
                            ElementState::Released => {
                                self.keys_released.insert(code);
                            }
                        };
                    }
                    _ => {}
                };
            }
            WindowEvent::ModifiersChanged(modifiers) => {
                self.modifiers = modifiers.state();
            }
            WindowEvent::MouseInput { state, button, .. } => {
                match state {
                    ElementState::Pressed => {
                        self.buttons_pressed.insert(*button);
                    }
                    ElementState::Released => {
                        self.buttons_released.insert(*button);
                    }
                };
            }
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

    pub fn is_key_pressed(&self, key_code: KeyCode) -> bool {
        self.keys_pressed.contains(&key_code)
    }

    pub fn is_key_released(&self, key_code: KeyCode) -> bool {
        self.keys_released.contains(&key_code)
    }

    pub fn is_button_pressed(&self, button: MouseButton) -> bool {
        self.buttons_pressed.contains(&button)
    }

    pub fn is_button_released(&self, button: MouseButton) -> bool {
        self.buttons_released.contains(&button)
    }

    pub fn get_mouse_position(&self) -> (f64, f64) {
        self.mouse_position
    }

    pub fn get_scroll_delta(&self) -> (f32, f32) {
        self.scroll_delta
    }
}
