use conrod_core::widget::envelope_editor::EnvelopePoint;
use conrod_core::{Colorable, Labelable, Positionable, Sizeable, Widget};
use std::collections::HashMap;
use winit::event_loop::ControlFlow;

use crate::audio::AudioContext;
use crate::database::Database;
use crate::gui::ConrodHandle;
use crate::input_manager::InputManager;
use crate::renderer::vertex::CoordColorVertex;
use crate::renderer::Renderer;
use crate::scene::main_menu_scene::play_bgm;
use crate::scene::{MaybeMessage, Scene, SceneOp, Value, MARGIN};
use crate::util::{any_sized_as_u8_slice, any_slice_as_u8_slice};
use crate::window::Window;
use conrod_core::widget_ids;

widget_ids! {
    pub struct SettingsSceneIds {
        // The main canvas
        canvas,

        header_canvas,
        title_text,

        body_canvas,

        settings_canvas,
        settings_canvas_scrollbar,

        footer_canvas,
        back_button,

        max_march_step_canvas,
        max_march_step_slider_label,
        max_march_step_slider,

        ambient_occlusion_sample_canvas,
        ambient_occlusion_sample_slider_label,
        ambient_occlusion_sample_slider,

        volume_canvas,
        volume_slider_label,
        volume_slider,
        volume_enable_box,

        crosshair_color_canvas,
        crosshair_color_label,
        crosshair_color_color_canvas,
        crosshair_color_r_canvas,
        crosshair_color_g_canvas,
        crosshair_color_b_canvas,
        crosshair_color_r_slider,
        crosshair_color_g_slider,
        crosshair_color_b_slider,
        crosshair_color_r_label,
        crosshair_color_g_label,
        crosshair_color_b_label,

        center_dot_enable_canvas,
        center_dot_enable_label,
        center_dot_enable_toggle,
        center_dot_thickness_canvas,
        center_dot_thickness_slider_label,
        center_dot_thickness_slider,
        // center_dot_offset_canvas,
        // center_dot_offset_slider_label,
        // center_dot_offset_slider,
        // center_dot_opacity_canvas,
        // center_dot_opacity_slider_label,
        // center_dot_opacity_slider,

        inner_line_enable_canvas,
        inner_line_enable_label,
        inner_line_enable_toggle,
        inner_line_thickness_canvas,
        inner_line_thickness_slider_label,
        inner_line_thickness_slider,
        inner_line_offset_canvas,
        inner_line_offset_slider_label,
        inner_line_offset_slider,
        // inner_line_opacity_canvas,
        // inner_line_opacity_slider_label,
        // inner_line_opacity_slider,

        outer_line_enable_canvas,
        outer_line_enable_label,
        outer_line_enable_toggle,
        outer_line_thickness_canvas,
        outer_line_thickness_slider_label,
        outer_line_thickness_slider,
        outer_line_offset_canvas,
        outer_line_offset_slider_label,
        outer_line_offset_slider,
        outer_line_opacity_canvas,
        outer_line_opacity_slider_label,
        outer_line_opacity_slider,

        crosshair_preview_canvas,
        crosshair_preview_image,
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

fn settings_item_canvas() -> conrod_core::widget::Canvas<'static> {
    conrod_core::widget::Canvas::new()
        .length(90.0)
        .pad_top(10.0)
        .pad_bottom(10.0)
}

impl Scene for SettingsScene {
    fn init(
        &mut self,
        message: MaybeMessage,
        _window: &mut Window,
        renderer: &mut Renderer,
        _conrod_handle: &mut ConrodHandle,
        audio_context: &mut AudioContext,
        database: &mut Database,
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
        database: &mut Database,
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
            .flow_down(&[
                (
                    self.ids.body_canvas,
                    conrod_core::widget::Canvas::new().flow_right(&[
                        (
                            self.ids.settings_canvas,
                            conrod_core::widget::Canvas::new()
                                .scroll_kids_vertically()
                                .parent(self.ids.canvas)
                                .flow_down(&[
                                    (self.ids.max_march_step_canvas, settings_item_canvas()),
                                    (
                                        self.ids.ambient_occlusion_sample_canvas,
                                        settings_item_canvas(),
                                    ),
                                    (
                                        self.ids.crosshair_color_canvas,
                                        conrod_core::widget::Canvas::new()
                                            .length(90.0)
                                            .pad_top(10.0)
                                            .pad_bottom(10.0)
                                            .flow_right(&[
                                                (
                                                    self.ids.crosshair_color_r_canvas,
                                                    conrod_core::widget::Canvas::new(),
                                                ),
                                                (
                                                    self.ids.crosshair_color_g_canvas,
                                                    conrod_core::widget::Canvas::new(),
                                                ),
                                                (
                                                    self.ids.crosshair_color_b_canvas,
                                                    conrod_core::widget::Canvas::new(),
                                                ),
                                            ]),
                                    ),
                                    (self.ids.volume_canvas, settings_item_canvas()),
                                    // Center dot
                                    (self.ids.center_dot_enable_canvas, settings_item_canvas()),
                                    // (self.ids.center_dot_opacity_canvas, settings_item_canvas()),
                                    (self.ids.center_dot_thickness_canvas, settings_item_canvas()),
                                    // Inner line
                                    (self.ids.inner_line_enable_canvas, settings_item_canvas()),
                                    // (self.ids.inner_line_opacity_canvas, settings_item_canvas()),
                                    (self.ids.inner_line_thickness_canvas, settings_item_canvas()),
                                    (self.ids.inner_line_offset_canvas, settings_item_canvas()),
                                    // Outer line
                                    (self.ids.outer_line_enable_canvas, settings_item_canvas()),
                                    // (self.ids.outer_line_opacity_canvas, settings_item_canvas()),
                                    (self.ids.outer_line_thickness_canvas, settings_item_canvas()),
                                    (self.ids.outer_line_offset_canvas, settings_item_canvas()),
                                ]),
                        ),
                        (
                            self.ids.crosshair_preview_canvas,
                            conrod_core::widget::Canvas::new().length(250.0),
                        ),
                    ]),
                ),
                (
                    self.ids.footer_canvas,
                    conrod_core::widget::Canvas::new().length(60.0),
                ),
            ])
            .set(self.ids.canvas, &mut ui_cell);

        conrod_core::widget::Scrollbar::y_axis(self.ids.settings_canvas)
            .rgb(1.0, 0.0, 0.0)
            .h_of(self.ids.settings_canvas)
            .set(self.ids.settings_canvas_scrollbar, &mut ui_cell);

        conrod_core::widget::Text::new("Settings")
            .font_id(ropa_font_id)
            .middle_of(self.ids.header_canvas)
            .parent(self.ids.header_canvas)
            .set(self.ids.title_text, &mut ui_cell);

        const GAP_BETWEEN_OPTION_SETTINGS: f64 = 30.0;
        const GAP_BETWEEN_LABEL_TO_OPTION_SETTINGS: f64 = 20.0;

        conrod_core::widget::Text::new("Maximum raymarch step")
            .font_id(ropa_font_id)
            .mid_top_of(self.ids.max_march_step_canvas)
            .set(self.ids.max_march_step_slider_label, &mut ui_cell);

        for value in conrod_core::widget::Slider::new(
            renderer.rendering_info.queuecount_raymarchmaxstep_aostep.y as f32,
            0f32,
            200f32,
        )
        .mid_bottom_of(self.ids.max_march_step_canvas)
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
            .mid_top_of(self.ids.ambient_occlusion_sample_canvas)
            .set(self.ids.ambient_occlusion_sample_slider_label, &mut ui_cell);

        for value in conrod_core::widget::Slider::new(
            renderer.rendering_info.queuecount_raymarchmaxstep_aostep.z as f32,
            0f32,
            10f32,
        )
        .mid_bottom_of(self.ids.ambient_occlusion_sample_canvas)
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
            .mid_top_of(self.ids.volume_canvas)
            .set(self.ids.volume_slider_label, &mut ui_cell);

        for value in conrod_core::widget::Slider::new(audio_context.get_volume(), 0f32, 1f32)
            .mid_bottom_of(self.ids.volume_canvas)
            .label(&format!("{}", (audio_context.get_volume() * 100.0) as u8))
            .wh(conrod_core::Dimensions::new(200.0, 30.0))
            .set(self.ids.volume_slider, &mut ui_cell)
        {
            audio_context.set_volume(value);
        }

        conrod_core::widget::Text::new("Center dot enabled")
            .font_id(ropa_font_id)
            .mid_top_of(self.ids.center_dot_enable_canvas)
            .set(self.ids.center_dot_enable_label, &mut ui_cell);

        for value in conrod_core::widget::Toggle::new(renderer.crosshair.center_dot_enabled)
            .mid_bottom_of(self.ids.center_dot_enable_canvas)
            .wh(conrod_core::Dimensions::new(40.0, 40.0))
            .set(self.ids.center_dot_enable_toggle, &mut ui_cell)
        {
            renderer.crosshair.center_dot_enabled = value;
        }

        // conrod_core::widget::Text::new("Center dot opacity")
        //     .font_id(ropa_font_id)
        //     .mid_top_of(self.ids.center_dot_opacity_canvas)
        //     .set(self.ids.center_dot_opacity_slider_label, &mut ui_cell);

        // for value in
        //     conrod_core::widget::Slider::new(renderer.crosshair.center_dot_opacity, 0f32, 1f32)
        //         .mid_bottom_of(self.ids.center_dot_opacity_canvas)
        //
        //         .label(&format!("{}", renderer.crosshair.center_dot_opacity))
        //         .wh(conrod_core::Dimensions::new(200.0, 30.0))
        //         .set(self.ids.center_dot_opacity_slider, &mut ui_cell)
        // {
        //     renderer.crosshair.center_dot_opacity = value;
        // }

        conrod_core::widget::Text::new("Center dot thickness")
            .font_id(ropa_font_id)
            .mid_top_of(self.ids.center_dot_thickness_canvas)
            .set(self.ids.center_dot_thickness_slider_label, &mut ui_cell);

        for value in
            conrod_core::widget::Slider::new(renderer.crosshair.center_dot_thickness, 0f32, 100f32)
                .mid_bottom_of(self.ids.center_dot_thickness_canvas)
                .label(&format!(
                    "{}",
                    renderer.crosshair.center_dot_thickness as u8
                ))
                .wh(conrod_core::Dimensions::new(200.0, 30.0))
                .set(self.ids.center_dot_thickness_slider, &mut ui_cell)
        {
            renderer.crosshair.center_dot_thickness = value.round();
        }

        conrod_core::widget::Text::new("Inner line enabled")
            .font_id(ropa_font_id)
            .mid_top_of(self.ids.inner_line_enable_canvas)
            .set(self.ids.inner_line_enable_label, &mut ui_cell);

        for value in conrod_core::widget::Toggle::new(renderer.crosshair.inner_line_enabled)
            .mid_bottom_of(self.ids.inner_line_enable_canvas)
            .wh(conrod_core::Dimensions::new(40.0, 40.0))
            .set(self.ids.inner_line_enable_toggle, &mut ui_cell)
        {
            renderer.crosshair.inner_line_enabled = value;
        }

        // conrod_core::widget::Text::new("Inner line opacity")
        //     .font_id(ropa_font_id)
        //     .mid_top_of(self.ids.inner_line_opacity_canvas)
        //     .set(self.ids.inner_line_opacity_slider_label, &mut ui_cell);
        //
        // for value in
        //     conrod_core::widget::Slider::new(renderer.crosshair.inner_line_opacity, 0f32, 1f32)
        //         .mid_bottom_of(self.ids.inner_line_opacity_canvas)
        //
        //         .label(&format!("{}", renderer.crosshair.inner_line_opacity))
        //         .wh(conrod_core::Dimensions::new(200.0, 30.0))
        //         .set(self.ids.inner_line_opacity_slider, &mut ui_cell)
        // {
        //     renderer.crosshair.inner_line_opacity = value;
        // }

        conrod_core::widget::Text::new("Inner line thickness")
            .font_id(ropa_font_id)
            .mid_top_of(self.ids.inner_line_thickness_canvas)
            .set(self.ids.inner_line_thickness_slider_label, &mut ui_cell);

        for value in
            conrod_core::widget::Slider::new(renderer.crosshair.inner_line_thickness, 0f32, 100f32)
                .mid_bottom_of(self.ids.inner_line_thickness_canvas)
                .label(&format!(
                    "{}",
                    renderer.crosshair.inner_line_thickness as u8
                ))
                .wh(conrod_core::Dimensions::new(200.0, 30.0))
                .set(self.ids.inner_line_thickness_slider, &mut ui_cell)
        {
            renderer.crosshair.inner_line_thickness = value.round();
        }

        conrod_core::widget::Text::new("Inner line offset")
            .font_id(ropa_font_id)
            .mid_top_of(self.ids.inner_line_offset_canvas)
            .set(self.ids.inner_line_offset_slider_label, &mut ui_cell);

        for value in
            conrod_core::widget::Slider::new(renderer.crosshair.inner_line_offset, 0f32, 100f32)
                .mid_bottom_of(self.ids.inner_line_offset_canvas)
                .label(&format!("{}", renderer.crosshair.inner_line_offset as u8))
                .wh(conrod_core::Dimensions::new(200.0, 30.0))
                .set(self.ids.inner_line_offset_slider, &mut ui_cell)
        {
            renderer.crosshair.inner_line_offset = value.round();
        }

        conrod_core::widget::Text::new("Outer line enabled")
            .font_id(ropa_font_id)
            .mid_top_of(self.ids.outer_line_enable_canvas)
            .set(self.ids.outer_line_enable_label, &mut ui_cell);

        for value in conrod_core::widget::Toggle::new(renderer.crosshair.outer_line_enabled)
            .mid_bottom_of(self.ids.outer_line_enable_canvas)
            .wh(conrod_core::Dimensions::new(40.0, 40.0))
            .set(self.ids.outer_line_enable_toggle, &mut ui_cell)
        {
            renderer.crosshair.outer_line_enabled = value;
        }

        // conrod_core::widget::Text::new("Outer line opacity")
        //     .font_id(ropa_font_id)
        //     .mid_top_of(self.ids.outer_line_opacity_canvas)
        //     .set(self.ids.outer_line_opacity_slider_label, &mut ui_cell);
        //
        // for value in
        //     conrod_core::widget::Slider::new(renderer.crosshair.outer_line_opacity, 0f32, 1f32)
        //         .mid_bottom_of(self.ids.outer_line_opacity_canvas)
        //
        //         .label(&format!("{}", renderer.crosshair.outer_line_opacity))
        //         .wh(conrod_core::Dimensions::new(200.0, 30.0))
        //         .set(self.ids.outer_line_opacity_slider, &mut ui_cell)
        // {
        //     renderer.crosshair.outer_line_opacity = value;
        // }

        conrod_core::widget::Text::new("Outer line thickness")
            .font_id(ropa_font_id)
            .mid_top_of(self.ids.outer_line_thickness_canvas)
            .set(self.ids.outer_line_thickness_slider_label, &mut ui_cell);

        for value in
            conrod_core::widget::Slider::new(renderer.crosshair.outer_line_thickness, 0f32, 100f32)
                .mid_bottom_of(self.ids.outer_line_thickness_canvas)
                .label(&format!(
                    "{}",
                    renderer.crosshair.outer_line_thickness as u8
                ))
                .wh(conrod_core::Dimensions::new(200.0, 30.0))
                .set(self.ids.outer_line_thickness_slider, &mut ui_cell)
        {
            renderer.crosshair.outer_line_thickness = value.round();
        }

        conrod_core::widget::Text::new("Outer line offset")
            .font_id(ropa_font_id)
            .mid_top_of(self.ids.outer_line_offset_canvas)
            .set(self.ids.outer_line_offset_slider_label, &mut ui_cell);

        for value in
            conrod_core::widget::Slider::new(renderer.crosshair.outer_line_offset, 0f32, 100f32)
                .mid_bottom_of(self.ids.outer_line_offset_canvas)
                .label(&format!("{}", renderer.crosshair.outer_line_offset as u8))
                .wh(conrod_core::Dimensions::new(200.0, 30.0))
                .set(self.ids.outer_line_offset_slider, &mut ui_cell)
        {
            renderer.crosshair.outer_line_offset = value.round();
        }

        conrod_core::widget::Text::new("Red color")
            .font_id(ropa_font_id)
            .padded_w_of(self.ids.crosshair_color_r_canvas, MARGIN)
            .center_justify()
            .mid_top_of(self.ids.crosshair_color_r_canvas)
            .set(self.ids.crosshair_color_r_label, &mut ui_cell);

        for value in conrod_core::widget::Slider::new(renderer.crosshair.color.x, 0f32, 1f32)
            .mid_bottom_of(self.ids.crosshair_color_r_canvas)
            .padded_w_of(self.ids.crosshair_color_r_canvas, MARGIN)
            .label(&format!("{:.3}", renderer.crosshair.color.x))
            .wh(conrod_core::Dimensions::new(200.0, 30.0))
            .set(self.ids.crosshair_color_r_slider, &mut ui_cell)
        {
            renderer.crosshair.color.x = value;
        }

        conrod_core::widget::Text::new("Green color")
            .font_id(ropa_font_id)
            .padded_w_of(self.ids.crosshair_color_r_canvas, MARGIN)
            .center_justify()
            .mid_top_of(self.ids.crosshair_color_g_canvas)
            .set(self.ids.crosshair_color_g_label, &mut ui_cell);

        for value in conrod_core::widget::Slider::new(renderer.crosshair.color.y, 0f32, 1f32)
            .mid_bottom_of(self.ids.crosshair_color_g_canvas)
            .padded_w_of(self.ids.crosshair_color_r_canvas, MARGIN)
            .label(&format!("{:.3}", renderer.crosshair.color.y))
            .wh(conrod_core::Dimensions::new(200.0, 30.0))
            .set(self.ids.crosshair_color_g_slider, &mut ui_cell)
        {
            renderer.crosshair.color.y = value;
        }

        conrod_core::widget::Text::new("Blue color")
            .font_id(ropa_font_id)
            .padded_w_of(self.ids.crosshair_color_r_canvas, MARGIN)
            .center_justify()
            .mid_top_of(self.ids.crosshair_color_b_canvas)
            .set(self.ids.crosshair_color_b_label, &mut ui_cell);

        for value in conrod_core::widget::Slider::new(renderer.crosshair.color.z, 0f32, 1f32)
            .mid_bottom_of(self.ids.crosshair_color_b_canvas)
            .padded_w_of(self.ids.crosshair_color_r_canvas, MARGIN)
            .label(&format!("{:.3}", renderer.crosshair.color.z))
            .wh(conrod_core::Dimensions::new(200.0, 30.0))
            .set(self.ids.crosshair_color_b_slider, &mut ui_cell)
        {
            renderer.crosshair.color.z = value;
        }

        conrod_core::widget::Image::new(crosshair_texture_id)
            .middle_of(self.ids.crosshair_preview_canvas)
            .wh(conrod_core::Dimensions::new(200.0, 200.0))
            .set(self.ids.crosshair_preview_image, &mut ui_cell);

        for _press in conrod_core::widget::Button::new()
            .label("Back")
            .bottom_left_with_margin_on(self.ids.footer_canvas, MARGIN)
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
        audio_context: &mut AudioContext,
        database: &mut Database,
    ) {
        database
            .glue
            .execute(&format!(
                "UPDATE settings SET \
                audio_volume = {},\
                maximum_raymarch_step = {},\
                ambient_occlusion_sample = {},\
                crosshair_color_r = {},\
                crosshair_color_g = {},\
                crosshair_color_b = {},\
                center_dot_enable = {},\
                center_dot_thickness = {},\
                inner_line_enable = {},\
                inner_line_thickness = {},\
                inner_line_length = {},\
                inner_line_offset = {},\
                outer_line_enable = {},\
                outer_line_thickness = {},\
                outer_line_length = {},\
                outer_line_offset = {}",
                audio_context.volume,
                renderer.rendering_info.queuecount_raymarchmaxstep_aostep.y,
                renderer.rendering_info.queuecount_raymarchmaxstep_aostep.z,
                renderer.crosshair.color.x,
                renderer.crosshair.color.y,
                renderer.crosshair.color.z,
                renderer.crosshair.center_dot_enabled,
                renderer.crosshair.center_dot_thickness,
                renderer.crosshair.inner_line_enabled,
                renderer.crosshair.inner_line_thickness,
                renderer.crosshair.inner_line_length,
                renderer.crosshair.inner_line_offset,
                renderer.crosshair.outer_line_enabled,
                renderer.crosshair.outer_line_thickness,
                renderer.crosshair.outer_line_length,
                renderer.crosshair.outer_line_offset,
            ))
            .unwrap();
    }
}
