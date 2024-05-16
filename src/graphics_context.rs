use std::sync::Arc;
use wgpu::ColorTargetState;
use winit::window::Window;

pub struct WgpuContext<'wnd> {
    surface: wgpu::Surface<'wnd>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    render_pipeline: wgpu::RenderPipeline
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

        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("wgpu_render_pipeline_layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[]
        });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("wgpu_render_pipeline"),
            layout: Some(&render_pipeline_layout),

            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
                compilation_options: wgpu::PipelineCompilationOptions::default()
            },

            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(ColorTargetState { 
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL
                })]
            }),

            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false
            },

            depth_stencil: None,

            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false
            },

            multiview: None
        });

        Self {
            surface,
            device,
            queue,
            config,
            render_pipeline
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
    }

    pub fn clear(&mut self, color: wgpu::Color) -> Result<(), wgpu::SurfaceError> {

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
                    load: wgpu::LoadOp::Clear(color),
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

    pub fn render_triangle_test(&mut self, color: wgpu::Color) -> Result<(), wgpu::SurfaceError> {
        
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("wgpu_render_command_encoder")
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("wgpu_render_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(color),
                        store: wgpu::StoreOp::Store
                    }
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.draw(0..3, 0..1); 
        } 

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}