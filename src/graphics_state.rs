use winit::window::Window;

pub struct GraphicsState<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
}

impl<'a> GraphicsState<'a> {

    pub async fn from_window(window: &'a Window) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
        let surface = instance.create_surface(window).unwrap();
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface)
            }
        ).await.unwrap();
        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("Render Device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default()
            }, 
            None
        ).await.unwrap();
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