use winit::event_loop::{ControlFlow, EventLoop};

use crate::app::GvizorApp;

pub mod app;
pub mod graphics_context;
pub mod test_color;


fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = GvizorApp::default();
    event_loop.run_app(&mut app).unwrap();
}
