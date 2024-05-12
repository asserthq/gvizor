use winit::{
    application::ApplicationHandler, 
    window::{Window, WindowId},
    event::*,
    event_loop::ActiveEventLoop
};

use crate::graphics_state::GraphicsState;

#[derive(Default)]
pub struct GvizorApp<'a> {
    window: Option<Window>,
    graphics: Option<GraphicsState<'a>>
}

impl<'a> ApplicationHandler for GvizorApp<'a> {

    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.window = Some(event_loop.create_window(Window::default_attributes()).unwrap());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => self.window.as_ref().unwrap().request_redraw(),

            WindowEvent::KeyboardInput { event, .. } => {
                match event.state {
                    ElementState::Pressed => event_loop.exit(),
                    _ => ()
                }
            }

            _ => ()
        }
    }
}