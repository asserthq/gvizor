use std::sync::Arc;
use winit::window::Window;

pub struct WgpuContext<'wnd> {
    surface: wgpu::Surface<'wnd>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
}

impl<'wnd> WgpuContext<'wnd> {

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

    pub fn resize(&mut self, width: u32, height: u32) {
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
    }

    pub fn clear(&mut self, r: u8, g:u8, b: u8) -> Result<(), wgpu::SurfaceError> {

        let (r, g, b) = (r as f64 / 256.0, g as f64 / 256.0, b as f64 / 256.0);

        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("wgpu_render_command_encoder")
        });

        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("wgpu_render_pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color { r, g, b, a: 1.0 }),
                    store: wgpu::StoreOp::Store
                }
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None
        });
        
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}