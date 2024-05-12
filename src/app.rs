use winit::{
    application::ApplicationHandler, 
    window::{Window, WindowId},
    event::*,
    event_loop::ActiveEventLoop
};

#[derive(Default)]
pub struct GvizorApp {
    window: Option<Window>
}

impl ApplicationHandler for GvizorApp {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.window = Some(event_loop.create_window(Window::default_attributes()).unwrap());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => self.window.as_ref().unwrap().request_redraw(),
            _ => ()
        }
    }
}