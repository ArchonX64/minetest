pub mod vertex;
pub mod state;
pub mod application;


use application::Application;

use winit::{ event_loop::EventLoop };

pub fn run() -> anyhow::Result<()>{
    env_logger::init();

    let event_loop = EventLoop::with_user_event().build()?;

    let mut app = Application::new();
    event_loop.run_app(&mut app)?;

    Ok(())
}