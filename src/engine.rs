use crate::{input::Input, timer::Timer};

use winit::{
    application::ApplicationHandler,
    event::{MouseButton, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::KeyCode,
    window::{Window, WindowId},
};

pub struct Engine {
    window: Option<Window>,
    timer: Timer,
    input: Input,
}

impl Engine {
    fn new() -> Self {
        Self {
            window: None,
            timer: Timer::new(),
            input: Input::new(),
        }
    }

    pub fn run() {
        let mut engine = Engine::new();

        let event_loop = EventLoop::new().unwrap();
        _ = event_loop.run_app(&mut engine);
    }
}

impl ApplicationHandler for Engine {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attrs = Window::default_attributes().with_title("Lumbricus");
        let window = event_loop.create_window(window_attrs).unwrap();
        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::RedrawRequested => {
                let _dt = self.timer.tick();

                if self.input.is_key_held(KeyCode::KeyW) {
                    println!("Held W");
                }

                if self.input.is_key_pressed(KeyCode::Space) {
                    println!("Pressed Space");
                }

                if self.input.is_key_released(KeyCode::Space) {
                    println!("Released Space");
                }

                if self.input.is_button_held(MouseButton::Left) {
                    println!("Held Mouse Left");
                }

                if self.input.is_button_pressed(MouseButton::Right) {
                    println!("Pressed Mouse Right");
                }

                if self.input.is_button_released(MouseButton::Right) {
                    println!("Released Mouse Right");
                }

                println!("Pos: {:?}", self.input.get_mouse_position());
                println!("Delta: {:?}", self.input.get_scroll_delta());

                // println!("DT: {}, FPS: {}", dt, self.timer.get_fps());
            }
            WindowEvent::KeyboardInput { event, .. } => {
                self.input.handle_keyboard_event(event);
            }
            WindowEvent::MouseInput { state, button, .. } => {
                self.input.handle_mouse_button(state, button);
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.input.handle_cursor_moved((position.x, position.y));
            }
            WindowEvent::MouseWheel { delta, .. } => {
                self.input.handle_scroll_delta(delta);
            }
            WindowEvent::CloseRequested => event_loop.exit(),
            _ => {}
        };
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        self.input.clear();
        self.window.as_ref().unwrap().request_redraw();
    }
}
