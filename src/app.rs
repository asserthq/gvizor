use std::sync::Arc;

use winit::{
    application::ApplicationHandler, 
    window::{Window, WindowId},
    event::*,
    event_loop::ActiveEventLoop,
    dpi
};

use crate::graphics_context::WgpuContext;

#[derive(Default)]
pub struct GvizorApp<'app> {
    window: Option<Arc<Window>>,
    graphics: Option<WgpuContext<'app>>
}

impl<'app> ApplicationHandler for GvizorApp<'app> {

    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.window = Some(Arc::new(
            event_loop.create_window(Window::default_attributes()).unwrap()
        ));
        self.graphics = Some(WgpuContext::from_window(self.window.as_ref().unwrap()));
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

            WindowEvent::Resized(size) => self.resize(size),

            _ => ()
        }
    }
}

impl<'app> GvizorApp<'app> {

    fn resize(&mut self, size: dpi::PhysicalSize<u32>) {
        if size.width > 0 && size.height > 0 {
            match self.graphics.as_mut() {
                Some(graphics) => graphics.resize(size.width, size.height),
                None => ()
            }
        }
    }
}