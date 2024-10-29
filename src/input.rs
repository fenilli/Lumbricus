use std::collections::HashMap;

use winit::{
    event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent},
    keyboard::{KeyCode, ModifiersState, PhysicalKey},
};

#[derive(Debug, PartialEq)]
pub struct InputState(u32);
impl InputState {
    const PRESSED: Self = Self(1 << 0);
    const RELEASED: Self = Self(1 << 1);
    const UP: Self = Self(1 << 2);
    const DOWN: Self = Self(1 << 3);
}

impl std::ops::BitAnd for InputState {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitAndAssign for InputState {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = Self(self.0 & rhs.0)
    }
}

impl std::ops::BitOr for InputState {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for InputState {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = Self(self.0 | rhs.0)
    }
}

impl std::ops::Not for InputState {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

pub struct Input {
    // Keyboard tracking
    keys: HashMap<KeyCode, InputState>,
    modifiers: ModifiersState,

    // Mouse tracking
    buttons: HashMap<MouseButton, InputState>,
    mouse_position: (f64, f64),
    scroll_delta: (f32, f32),
}

#[allow(dead_code)]
impl Input {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
            modifiers: ModifiersState::empty(),

            buttons: HashMap::new(),
            mouse_position: (0.0, 0.0),
            scroll_delta: (0.0, 0.0),
        }
    }

    pub fn clear(&mut self) {
        for (_, input_state) in self.keys.iter_mut() {
            *input_state &= !(InputState::PRESSED | InputState::RELEASED);
        }
        for (_, input_state) in self.buttons.iter_mut() {
            *input_state &= !(InputState::PRESSED | InputState::RELEASED);
        }
        self.scroll_delta = (0.0, 0.0);
    }

    pub fn update(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                match event.physical_key {
                    PhysicalKey::Code(code) => {
                        let input_state = self.keys.entry(code).or_insert(InputState::UP);

                        match event.state {
                            ElementState::Pressed => {
                                *input_state = InputState::PRESSED | InputState::DOWN;
                            }
                            ElementState::Released => {
                                *input_state = InputState::RELEASED | InputState::UP;
                            }
                        };
                    }
                    _ => {}
                };
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let input_state = self.buttons.entry(*button).or_insert(InputState::UP);

                match state {
                    ElementState::Pressed => {
                        *input_state = InputState::PRESSED | InputState::DOWN;
                    }
                    ElementState::Released => {
                        *input_state = InputState::RELEASED | InputState::UP;
                    }
                };
            }
            WindowEvent::ModifiersChanged(modifiers) => {
                self.modifiers = modifiers.state();
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
        self.keys.get(&key_code).map_or(false, |input_state| {
            input_state.0 & InputState::PRESSED.0 == InputState::PRESSED.0
        })
    }

    pub fn is_key_released(&self, key_code: KeyCode) -> bool {
        self.keys.get(&key_code).map_or(false, |input_state| {
            input_state.0 & InputState::RELEASED.0 == InputState::RELEASED.0
        })
    }

    pub fn is_button_pressed(&self, button: MouseButton) -> bool {
        self.buttons.get(&button).map_or(false, |input_state| {
            input_state.0 & InputState::PRESSED.0 == InputState::PRESSED.0
        })
    }

    pub fn is_button_released(&self, button: MouseButton) -> bool {
        self.buttons.get(&button).map_or(false, |input_state| {
            input_state.0 & InputState::RELEASED.0 == InputState::RELEASED.0
        })
    }

    pub fn get_mouse_position(&self) -> (f64, f64) {
        self.mouse_position
    }

    pub fn get_scroll_delta(&self) -> (f32, f32) {
        self.scroll_delta
    }
}
