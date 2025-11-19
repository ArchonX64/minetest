use std::{collections::HashSet, sync::Arc};
use std::time::Instant;

use crate::graphics::Graphics;
use crate::game::Game;

use winit::application::ApplicationHandler;
use winit::dpi::{LogicalSize};
use winit::event::*;
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Fullscreen, Window, WindowId};

pub struct Application {
    graphics: Option<Graphics>,
    game: Game,

    pressed_keys: HashSet<KeyCode>,
    alpha: f64,
    mouse_x: f64,
    mouse_y: f64,
    mouse_dx: f64,
    mouse_dy: f64
}

pub struct Input {
    pub pressed_keys: Vec<KeyCode>,
    pub mouse_x: f64,
    pub mouse_y: f64,
    pub mouse_dx: f64,
    pub mouse_dy: f64,
}

impl Application {
    pub fn new() -> Self {
        let game = Game::new();

        Self {
            graphics: None,
            pressed_keys: HashSet::new(),
            game,
            alpha: 0.5,
            mouse_x: 0.,
            mouse_y: 0.,
            mouse_dx: 0.,
            mouse_dy: 0.
        }
    }

    pub fn get_input(&self) -> Input {
        Input {
            pressed_keys: self.pressed_keys.iter().copied().collect(),
            mouse_x: self.mouse_x,
            mouse_y: self.mouse_y,
            mouse_dx: self.mouse_dx,
            mouse_dy: self.mouse_dy
        }
    }

    pub fn request_redraw(&self) {
        match &self.graphics {
            Some(graphics) => graphics.window.request_redraw(),
            None => {}
        }
    }

    pub fn on_ready(&mut self) {
        self.game.reset_deltatime(); // Reset to prevent deltatime accumulation during loading
    }
}

impl ApplicationHandler<Graphics> for Application {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
            #[allow(unused_mut)]
            let mut window_attributes = Window::default_attributes()
                .with_inner_size(LogicalSize::new(1200, 800))
                .with_fullscreen(Some(Fullscreen::Borderless(None)));

            let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
            window.set_cursor_grab(winit::window::CursorGrabMode::Locked).unwrap();
            window.set_cursor_visible(false);

            self.graphics = Some(pollster::block_on(Graphics::new(window)).unwrap());
            self.on_ready();
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        // Trigger rendering
        match &self.graphics {
            Some(graphics) => {
                self.game.tick(self.get_input());  // Trigger game loop
                graphics.window.request_redraw();
            },
            None => {}
        }

        // Reset mouse dx and dy
        self.mouse_dx = 0.;
        self.mouse_dy = 0.;
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: Graphics) {
        self.graphics = Some(event);
    }

    fn device_event(&mut self, _event_loop: &ActiveEventLoop, _device_id: DeviceId, event: DeviceEvent) {
        match event {
            DeviceEvent::MouseMotion { 
                delta 
            } => {
                self.mouse_x += delta.0;
                self.mouse_y += delta.1;
                self.mouse_dx += delta.0;
                self.mouse_dy += delta.1;
            },
            _ => {}
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        let input = self.get_input();
        let graphics = match &mut self.graphics {
            Some(graphics) => graphics,
            None => return,
        };


        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => graphics.resize(size.width, size.height),
            WindowEvent::RedrawRequested => {
                match graphics.render(&mut self.game.get_renderables()) {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        let size = graphics.window.inner_size();
                        graphics.resize(size.width, size.height);
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
                }
                if key_state == ElementState::Released {
                    self.pressed_keys.remove(&code);
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

