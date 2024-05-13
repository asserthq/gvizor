use std::sync::Arc;

use winit::{
    application::ApplicationHandler, 
    dpi, 
    event::*, 
    event_loop::ActiveEventLoop, 
    window::{Window, WindowId}
};

use crate::graphics_context::WgpuContext;
use crate::test_color::CursorDependColor;

#[derive(Default)]
pub struct GvizorApp<'app> {
    window: Option<Arc<Window>>,
    graphics: Option<WgpuContext<'app>>,
    bg_color: wgpu::Color
}

impl<'app> ApplicationHandler for GvizorApp<'app> {

    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.window = Some(Arc::new(
            event_loop.create_window(Window::default_attributes()).unwrap()
        ));
        self.graphics = Some(WgpuContext::from_window(self.window.as_ref().unwrap()));
        self.window.as_ref().unwrap().request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        let window = self.window.as_ref().unwrap();
        let graphics = self.graphics.as_mut().unwrap();

        if window_id == window.id() {
            match event {
                WindowEvent::CloseRequested => event_loop.exit(),
                WindowEvent::RedrawRequested => {
                    match graphics.clear(self.bg_color) {
                        Ok(_) => (),
                        Err(wgpu::SurfaceError::Lost) => self.resize(window.inner_size()),
                        Err(wgpu::SurfaceError::OutOfMemory) => event_loop.exit(),
                        Err(err) => eprintln!("{:?}", err)
                    }
                },
    
                WindowEvent::KeyboardInput { event, .. } => {
                    match event.state {
                        ElementState::Pressed => event_loop.exit(),
                        _ => ()
                    }
                }
    
                WindowEvent::Resized(size) => self.resize(size),
                WindowEvent::Moved(_) => window.request_redraw(),

                WindowEvent::CursorMoved { position, .. } => {
                    self.bg_color.update(position.x, position.y, window.inner_size().width, window.inner_size().height);
                    window.request_redraw();
                },
    
                _ => ()
            }
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