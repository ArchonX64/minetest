use std::{collections::HashSet, sync::Arc};

use crate::graphics::state::State;

use winit::{ application::ApplicationHandler, dpi::{LogicalSize, Size}, event::*, event_loop::ActiveEventLoop, keyboard::{KeyCode, PhysicalKey}, window::{Fullscreen, Window}};

pub struct Application {
    state: Option<State>,
    pressed_keys: HashSet<KeyCode>,
    mouse_dx: f64,
    mouse_dy: f64
}

pub struct Input {
    pub pressed_keys: Vec<KeyCode>,
    pub mouse_dx: f64,
    pub mouse_dy: f64
}

impl Application {
    pub fn new() -> Self {
        Self {
            state: None,
            pressed_keys: HashSet::new(),
            mouse_dx: 0.,
            mouse_dy: 0.,
        }
    }
}

impl ApplicationHandler<State> for Application {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
            #[allow(unused_mut)]
            let mut window_attributes = Window::default_attributes()
                .with_inner_size(LogicalSize::new(1200, 800))
                .with_fullscreen(Some(Fullscreen::Borderless(None)));

            let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
            window.set_cursor_grab(winit::window::CursorGrabMode::Locked).unwrap();
            window.set_cursor_visible(false);

            self.state = Some(pollster::block_on(State::new(window)).unwrap());
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: State) {
        self.state = Some(event);
    }

    fn device_event(
            &mut self,
            _event_loop: &ActiveEventLoop,
            _device_id: DeviceId,
            event: DeviceEvent,
        ) {


        let state = match &mut self.state {
            Some(canvas) => canvas,
            None => return
        };
        
        match event {
            DeviceEvent::MouseMotion { 
                delta 
            } => {
                state.on_mouse_move(delta.0, delta.1);
            },
            _ => {}
        }
    }

    fn window_event(
            &mut self,
            event_loop: &ActiveEventLoop,
            _window_id: winit::window::WindowId,
            event: WindowEvent,
        ) {
        
        let state = match &mut self.state {
            Some(canvas) => canvas,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => state.resize(size.width, size.height),
            WindowEvent::RedrawRequested => {
                let input = Input {
                    pressed_keys: self.pressed_keys.iter().copied().collect(),
                    mouse_dx: self.mouse_dx,
                    mouse_dy: self.mouse_dy
                };

                match state.render(input) {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        let size = state.window.inner_size();
                        state.resize(size.width, size.height)
                    }
                    Err(e) => {
                        log::error!("Unable to render {}", e);
                    }
                }
            },
            WindowEvent::KeyboardInput { 
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
             } => {
                if key_state == ElementState::Pressed {
                    self.pressed_keys.insert(code);

                    state.on_key_press(code);
                }
                if key_state == ElementState::Released {
                    self.pressed_keys.remove(&code);

                    state.on_key_release(code);
                }

                match code {
                    KeyCode::Escape => {
                        event_loop.exit()
                    },
                    _ => {}
                }
             },
             _ => {}
        }
    }
}

