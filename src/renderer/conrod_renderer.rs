use crate::gui::ConrodHandle;
use crate::renderer::SurfaceAndWindowConfig;

pub struct ConrodSceneRenderer {
    pub conrod_renderer: conrod_wgpu::Renderer,
}

impl ConrodSceneRenderer {
    pub fn new(
        surface_config: &wgpu::SurfaceConfiguration,
        device: &wgpu::Device,
        _queue: &mut wgpu::Queue,
    ) -> Self {
        Self {
            conrod_renderer: conrod_wgpu::Renderer::new(device, 1, surface_config.format),
        }
    }

    pub fn render(
        &mut self,
        view: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        device: &wgpu::Device,
        surface_config: &SurfaceAndWindowConfig,
        conrod_handle: &mut ConrodHandle,
        on_top_of_game: bool,
    ) {
        {
            let primitives = conrod_handle.get_ui().draw();
            if let Some(cmd) = self
                .conrod_renderer
                .fill(
                    conrod_handle.get_image_map(),
                    [
                        0.0,
                        0.0,
                        surface_config.surface.width as f32,
                        surface_config.surface.height as f32,
                    ],
                    surface_config.window_scale_factor,
                    primitives,
                )
                .unwrap()
            {
                cmd.load_buffer_and_encode(device, encoder);
            }
        }
        let render = self
            .conrod_renderer
            .render(device, conrod_handle.get_image_map());
        let buffer_slice = render.vertex_buffer.slice(..);
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render pass gui"),
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: if on_top_of_game {
                        wgpu::LoadOp::Load
                    } else {
                        wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        })
                    },
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });
        let slot = 0;
        render_pass.set_vertex_buffer(slot, buffer_slice);
        for cmd in render.commands {
            match cmd {
                conrod_wgpu::RenderPassCommand::SetPipeline { pipeline } => {
                    render_pass.set_pipeline(pipeline);
                }
                conrod_wgpu::RenderPassCommand::SetBindGroup { bind_group } => {
                    render_pass.set_bind_group(0, bind_group, &[]);
                }
                conrod_wgpu::RenderPassCommand::SetScissor {
                    top_left: [_x, _y],
                    dimensions: [w, h],
                } => {
                    render_pass.set_scissor_rect(0, 0, w, h);
                }
                conrod_wgpu::RenderPassCommand::Draw { vertex_range } => {
                    render_pass.draw(vertex_range, 0..1);
                }
            }
        }
    }
}
