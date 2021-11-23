use conrod_core::widget::envelope_editor::EnvelopePoint;
use conrod_core::{Labelable, Positionable, Sizeable, Widget};
use winit::event_loop::ControlFlow;

use crate::audio::AudioContext;
use crate::gui::ConrodHandle;
use crate::input_manager::InputManager;
use crate::renderer::Renderer;
use crate::scene::{Scene, SceneOp, MARGIN};
use crate::window::Window;
use conrod_core::widget_ids;

widget_ids! {
    pub struct SettingsSceneIds {
        // The main canvas
        canvas,
        title_text,
        max_march_step_slider_label,
        max_march_step_slider,
        ambient_occlusion_sample_slider_label,
        ambient_occlusion_sample_slider,
        volume_slider_label,
        volume_slider,
        volume_tick_box,
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
        window: &mut Window,
        renderer: &mut Renderer,
        conrod_handle: &mut ConrodHandle,
        audio_context: &mut AudioContext,
    ) {
        renderer.is_render_gui = true;
        renderer.is_render_game = false;
    }

    fn update(
        &mut self,
        renderer: &mut Renderer,
        input_manager: &InputManager,
        delta_time: f32,
        conrod_handle: &mut ConrodHandle,
        audio_context: &mut AudioContext,
        control_flow: &mut ControlFlow,
    ) -> SceneOp {
        let mut scene_op = SceneOp::None;

        let ropa_font_id = *conrod_handle.get_font_id_map().get("ropa").unwrap();
        let mut ui_cell = conrod_handle.get_ui_mut().set_widgets();
        conrod_core::widget::Canvas::new()
            .pad(MARGIN)
            .set(self.ids.canvas, &mut ui_cell);

        conrod_core::widget::Text::new("Settings")
            .font_id(ropa_font_id)
            .align_middle_x()
            .mid_top_with_margin_on(self.ids.canvas, MARGIN)
            .set(self.ids.title_text, &mut ui_cell);

        const GAP_BETWEEN_OPTION_SETTINGS: f64 = 30.0;
        const GAP_BETWEEN_LABEL_TO_OPTION_SETTINGS: f64 = 20.0;

        conrod_core::widget::Text::new("Maximum raymarch step")
            .font_id(ropa_font_id)
            .align_middle_x()
            .down_from(self.ids.title_text, 50.0)
            .set(self.ids.max_march_step_slider_label, &mut ui_cell);

        for value in conrod_core::widget::Slider::new(
            renderer.rendering_info.queuecount_raymarchmaxstep_aostep.y as f32,
            0f32,
            200f32,
        )
        .align_middle_x()
        .label_font_id(ropa_font_id)
        .label(&format!(
            "{}",
            renderer.rendering_info.queuecount_raymarchmaxstep_aostep.y as u8
        ))
        .down_from(
            self.ids.max_march_step_slider_label,
            GAP_BETWEEN_LABEL_TO_OPTION_SETTINGS,
        )
        .wh(conrod_core::Dimensions::new(200.0, 30.0))
        .set(self.ids.max_march_step_slider, &mut ui_cell)
        {
            renderer.rendering_info.queuecount_raymarchmaxstep_aostep.y = value.round() as u32;
        }

        conrod_core::widget::Text::new("Ambient occlusion step")
            .font_id(ropa_font_id)
            .align_middle_x()
            .down_from(self.ids.max_march_step_slider, GAP_BETWEEN_OPTION_SETTINGS)
            .set(self.ids.ambient_occlusion_sample_slider_label, &mut ui_cell);

        for value in conrod_core::widget::Slider::new(
            renderer.rendering_info.queuecount_raymarchmaxstep_aostep.z as f32,
            0f32,
            10f32,
        )
        .align_middle_x()
        .label_font_id(ropa_font_id)
        .label(&format!(
            "{}",
            renderer.rendering_info.queuecount_raymarchmaxstep_aostep.z as u8
        ))
        .down_from(
            self.ids.ambient_occlusion_sample_slider_label,
            GAP_BETWEEN_LABEL_TO_OPTION_SETTINGS,
        )
        .wh(conrod_core::Dimensions::new(200.0, 30.0))
        .set(self.ids.ambient_occlusion_sample_slider, &mut ui_cell)
        {
            renderer.rendering_info.queuecount_raymarchmaxstep_aostep.z = value.round() as u32;
        }

        conrod_core::widget::Text::new("Volume")
            .font_id(ropa_font_id)
            .align_middle_x()
            .down_from(
                self.ids.ambient_occlusion_sample_slider,
                GAP_BETWEEN_OPTION_SETTINGS,
            )
            .set(self.ids.volume_slider_label, &mut ui_cell);

        for value in conrod_core::widget::Slider::new(audio_context.get_volume(), 0f32, 1f32)
            .align_middle_x()
            .label_font_id(ropa_font_id)
            .label(&format!("{}", (audio_context.get_volume() * 100.0) as u8))
            .down_from(
                self.ids.volume_slider_label,
                GAP_BETWEEN_LABEL_TO_OPTION_SETTINGS,
            )
            .wh(conrod_core::Dimensions::new(200.0, 30.0))
            .set(self.ids.volume_slider, &mut ui_cell)
        {
            audio_context.set_volume(value);
        }

        for _press in conrod_core::widget::Button::new()
            .label("Back")
            .label_font_id(ropa_font_id)
            .bottom_left_with_margin_on(self.ids.canvas, MARGIN)
            .wh(conrod_core::Dimensions::new(100.0, 30.0))
            .set(self.ids.back_button, &mut ui_cell)
        {
            scene_op = SceneOp::Pop(1);
        }

        scene_op
    }

    fn deinit(
        &mut self,
        window: &mut Window,
        renderer: &mut Renderer,
        conrod_handle: &mut ConrodHandle,
        audio_context: &mut AudioContext,
    ) {
    }
}
