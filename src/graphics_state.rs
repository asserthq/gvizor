use std::sync::Arc;
use winit::window::Window;

pub struct GraphicsState<'wnd> {
    surface: wgpu::Surface<'wnd>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
}

impl<'wnd> GraphicsState<'wnd> {

    pub fn from_window(window: &Arc<Window>) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
        let surface = instance.create_surface(window.clone()).unwrap();
        let adapter = futures::executor::block_on(
            instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface)
            }
        )).unwrap();
        let (device, queue) = futures::executor::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("Render Device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default()
            }, 
            None
        )).unwrap();
        let size = window.inner_size();
        let config = surface.get_default_config(&adapter, size.width, size.height).unwrap();

        Self {
            surface,
            device,
            queue,
            config
        }
    }
}