pub mod graphics;
mod application;
mod util;
mod game;

use winit::event_loop::EventLoop;
use application::Application;

pub fn main() {
    env_logger::init();

    let event_loop = EventLoop::with_user_event().build().unwrap();

    let mut app = Application::new();

    event_loop.run_app(&mut app).unwrap();
}