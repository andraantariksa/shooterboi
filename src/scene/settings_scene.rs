use conrod_core::widget::envelope_editor::EnvelopePoint;
use conrod_core::{Colorable, Labelable, Positionable, Sizeable, Widget};
use std::collections::HashMap;
use winit::event_loop::ControlFlow;

use crate::audio::AudioContext;
use crate::gui::ConrodHandle;
use crate::input_manager::InputManager;
use crate::renderer::vertex::CoordColorVertex;
use crate::renderer::Renderer;
use crate::scene::{MaybeMessage, Scene, SceneOp, Value, MARGIN};
use crate::util::{any_sized_as_u8_slice, any_slice_as_u8_slice};
use crate::window::Window;
use conrod_core::widget_ids;

widget_ids! {
    pub struct SettingsSceneIds {
        // The main canvas
        canvas,
        settings_canvas,

        title_text,

        settings_max_march_step_slider_canvas,

        max_march_step_slider_label,
        max_march_step_slider,

        ambient_occlusion_sample_slider_label,
        ambient_occlusion_sample_slider,

        volume_slider_label,
        volume_slider,
        volume_enable_box,

        crosshair_color_label,
        crosshair_color_canvas,
        crosshair_color_r_canvas,
        crosshair_color_g_canvas,
        crosshair_color_b_canvas,

        center_dot_enable_box,
        center_dot_thickness_slider_label,
        center_dot_thickness_slider,
        center_dot_opacity_slider_label,
        center_dot_opacity_slider,

        inner_line_enable_box,
        inner_line_thickness_slider_label,
        inner_line_thickness_slider,
        inner_line_opacity_slider_label,
        inner_line_opacity_slider,

        crosshair_preview_image,

        back_button
    }
}

pub struct SettingsScene {
    ids: SettingsSceneIds,
}

impl SettingsScene {
    pub fn new(renderer: &mut Renderer, conrod_handle: &mut ConrodHandle) -> Self {
        Self {
            ids: SettingsSceneIds::new(conrod_handle.get_ui_mut().widget_id_generator()),
        }
    }
}

impl Scene for SettingsScene {
    fn init(
        &mut self,
        _message: MaybeMessage,
        _window: &mut Window,
        renderer: &mut Renderer,
        _conrod_handle: &mut ConrodHandle,
        _audio_context: &mut AudioContext,
    ) {
        renderer.is_render_gui = true;
        renderer.is_render_game = false;

        renderer.rendering_info.reso_time.x = 200.0f32;
        renderer.rendering_info.reso_time.y = 200.0f32;
        renderer.queue.write_buffer(
            &renderer.game_renderer.rendering_info_buffer,
            0,
            any_sized_as_u8_slice(&renderer.rendering_info),
        );
    }

    fn update(
        &mut self,
        window: &mut Window,
        renderer: &mut Renderer,
        _input_manager: &InputManager,
        _delta_time: f32,
        conrod_handle: &mut ConrodHandle,
        audio_context: &mut AudioContext,
        _control_flow: &mut ControlFlow,
    ) -> SceneOp {
        let crosshair_texture_id = conrod_handle
            .get_image_id_map()
            .get("crosshair")
            .unwrap()
            .clone();
        let crosshair_image = conrod_handle
            .get_image_map()
            .get(&crosshair_texture_id)
            .unwrap();
        let crosshair_texture_view =
            crosshair_image
                .texture
                .create_view(&wgpu::TextureViewDescriptor {
                    label: Some("Texture view descriptor crosshair"),
                    format: Some(renderer.surface_and_window_config.surface.format),
                    ..Default::default()
                });
        let mut encoder = renderer
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render encoder"),
            });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Game render pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &crosshair_texture_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });
            render_pass.set_bind_group(0, &renderer.game_renderer.main_bind_group, &[]);
            render_pass.set_pipeline(&renderer.game_renderer.crosshair_render_pipeline);
            render_pass
                .set_vertex_buffer(0, renderer.game_renderer.crosshair_vertex_buffer.slice(..));
            render_pass.draw(0..renderer.crosshair.vertices_len(), 0..1);
        }
        renderer.queue.submit(core::iter::once(encoder.finish()));

        let mut scene_op = SceneOp::None;

        let ropa_font_id = *conrod_handle.get_font_id_map().get("ropa").unwrap();
        let mut ui_cell = conrod_handle.get_ui_mut().set_widgets();
        conrod_core::widget::Canvas::new()
            .pad(MARGIN)
            .scroll_kids_vertically()
            .rgb(0.8, 0.8, 0.0)
            .set(self.ids.canvas, &mut ui_cell);

        conrod_core::widget::Text::new("Settings")
            .font_id(ropa_font_id)
            .align_middle_x_of(self.ids.canvas)
            .mid_top_with_margin_on(self.ids.canvas, MARGIN)
            .set(self.ids.title_text, &mut ui_cell);

        const GAP_BETWEEN_OPTION_SETTINGS: f64 = 30.0;
        const GAP_BETWEEN_LABEL_TO_OPTION_SETTINGS: f64 = 20.0;

        conrod_core::widget::Text::new("Maximum raymarch step")
            .font_id(ropa_font_id)
            .align_middle_x_of(self.ids.canvas)
            .set(self.ids.max_march_step_slider_label, &mut ui_cell);

        for value in conrod_core::widget::Slider::new(
            renderer.rendering_info.queuecount_raymarchmaxstep_aostep.y as f32,
            0f32,
            200f32,
        )
        .align_middle_x_of(self.ids.canvas)
        .label_font_id(ropa_font_id)
        .label(&format!(
            "{}",
            renderer.rendering_info.queuecount_raymarchmaxstep_aostep.y as u8
        ))
        .wh(conrod_core::Dimensions::new(200.0, 30.0))
        .set(self.ids.max_march_step_slider, &mut ui_cell)
        {
            renderer.rendering_info.queuecount_raymarchmaxstep_aostep.y = value.round() as u32;
        }

        conrod_core::widget::Text::new("Ambient occlusion step")
            .font_id(ropa_font_id)
            .align_middle_x_of(self.ids.canvas)
            .set(self.ids.ambient_occlusion_sample_slider_label, &mut ui_cell);

        for value in conrod_core::widget::Slider::new(
            renderer.rendering_info.queuecount_raymarchmaxstep_aostep.z as f32,
            0f32,
            10f32,
        )
        .align_middle_x_of(self.ids.canvas)
        .label_font_id(ropa_font_id)
        .label(&format!(
            "{}",
            renderer.rendering_info.queuecount_raymarchmaxstep_aostep.z as u8
        ))
        .wh(conrod_core::Dimensions::new(200.0, 30.0))
        .set(self.ids.ambient_occlusion_sample_slider, &mut ui_cell)
        {
            renderer.rendering_info.queuecount_raymarchmaxstep_aostep.z = value.round() as u32;
        }

        conrod_core::widget::Text::new("Audio Volume")
            .font_id(ropa_font_id)
            .align_middle_x_of(self.ids.canvas)
            .set(self.ids.volume_slider_label, &mut ui_cell);

        for value in conrod_core::widget::Slider::new(audio_context.get_volume(), 0f32, 1f32)
            .align_middle_x_of(self.ids.canvas)
            .label_font_id(ropa_font_id)
            .label(&format!("{}", (audio_context.get_volume() * 100.0) as u8))
            .wh(conrod_core::Dimensions::new(200.0, 30.0))
            .set(self.ids.volume_slider, &mut ui_cell)
        {
            audio_context.set_volume(value);
        }

        conrod_core::widget::Text::new("Center dot opacity")
            .font_id(ropa_font_id)
            .align_middle_x_of(self.ids.canvas)
            .set(self.ids.crosshair_color_label, &mut ui_cell);

        conrod_core::widget::Canvas::new()
            .flow_right(&[
                (
                    self.ids.crosshair_color_r_canvas,
                    conrod_core::widget::Canvas::new().rgb(1.0, 0.0, 0.0),
                ),
                (
                    self.ids.crosshair_color_g_canvas,
                    conrod_core::widget::Canvas::new().rgb(0.0, 1.0, 0.0),
                ),
                (
                    self.ids.crosshair_color_b_canvas,
                    conrod_core::widget::Canvas::new().rgb(0.0, 0.0, 1.0),
                ),
            ])
            .rgb(0.5, 0.5, 0.5)
            .set(self.ids.crosshair_color_canvas, &mut ui_cell);

        conrod_core::widget::Text::new("Center dot opacity")
            .font_id(ropa_font_id)
            .align_middle_x_of(self.ids.canvas)
            .set(self.ids.center_dot_opacity_slider_label, &mut ui_cell);

        for value in
            conrod_core::widget::Slider::new(renderer.crosshair.center_dot_opacity, 0f32, 1f32)
                .align_middle_x_of(self.ids.canvas)
                .label_font_id(ropa_font_id)
                .label(&format!("{}", renderer.crosshair.center_dot_opacity))
                .wh(conrod_core::Dimensions::new(200.0, 30.0))
                .set(self.ids.center_dot_opacity_slider, &mut ui_cell)
        {
            renderer.crosshair.center_dot_opacity = value;
        }

        conrod_core::widget::Text::new("Center dot thickness")
            .font_id(ropa_font_id)
            .align_middle_x_of(self.ids.canvas)
            .set(self.ids.center_dot_thickness_slider_label, &mut ui_cell);

        for value in conrod_core::widget::Slider::new(audio_context.get_volume(), 0f32, 100f32)
            .align_middle_x_of(self.ids.canvas)
            .label_font_id(ropa_font_id)
            .label(&format!(
                "{}",
                renderer.crosshair.center_dot_thickness as u8
            ))
            .wh(conrod_core::Dimensions::new(200.0, 30.0))
            .set(self.ids.center_dot_thickness_slider, &mut ui_cell)
        {
            renderer.crosshair.center_dot_thickness = value.round();
        }

        conrod_core::widget::Text::new("Inner line opacity")
            .font_id(ropa_font_id)
            .align_middle_x_of(self.ids.canvas)
            .set(self.ids.inner_line_opacity_slider_label, &mut ui_cell);

        for value in
            conrod_core::widget::Slider::new(renderer.crosshair.inner_line_opacity, 0f32, 1f32)
                .align_middle_x_of(self.ids.canvas)
                .label_font_id(ropa_font_id)
                .label(&format!("{}", renderer.crosshair.inner_line_opacity))
                .wh(conrod_core::Dimensions::new(200.0, 30.0))
                .set(self.ids.inner_line_opacity_slider, &mut ui_cell)
        {
            renderer.crosshair.inner_line_opacity = value;
        }

        conrod_core::widget::Text::new("Inner line thickness")
            .font_id(ropa_font_id)
            .align_middle_x_of(self.ids.canvas)
            .set(self.ids.inner_line_thickness_slider_label, &mut ui_cell);

        for value in
            conrod_core::widget::Slider::new(renderer.crosshair.inner_line_thickness, 0f32, 100f32)
                .align_middle_x_of(self.ids.canvas)
                .label_font_id(ropa_font_id)
                .label(&format!(
                    "{}",
                    renderer.crosshair.inner_line_thickness as u8
                ))
                .wh(conrod_core::Dimensions::new(200.0, 30.0))
                .set(self.ids.inner_line_thickness_slider, &mut ui_cell)
        {
            renderer.crosshair.inner_line_thickness = value.round();
        }

        conrod_core::widget::Image::new(crosshair_texture_id)
            .bottom_right_with_margin_on(self.ids.canvas, MARGIN)
            .wh(conrod_core::Dimensions::new(200.0, 200.0))
            .set(self.ids.crosshair_preview_image, &mut ui_cell);

        for _press in conrod_core::widget::Button::new()
            .label("Back")
            .label_font_id(ropa_font_id)
            .bottom_left_with_margin_on(self.ids.canvas, MARGIN)
            .wh(conrod_core::Dimensions::new(100.0, 30.0))
            .set(self.ids.back_button, &mut ui_cell)
        {
            scene_op = SceneOp::Pop(1, {
                let mut m = HashMap::new();
                m.insert("start_bgm", Value::Bool(false));
                Some(m)
            });
        }

        renderer.crosshair.update_vertices(
            &renderer.queue,
            &renderer.game_renderer.crosshair_vertex_buffer,
            0 as wgpu::BufferAddress,
        );

        scene_op
    }

    fn deinit(
        &mut self,
        _window: &mut Window,
        renderer: &mut Renderer,
        _conrod_handle: &mut ConrodHandle,
        _audio_context: &mut AudioContext,
    ) {
    }
}
