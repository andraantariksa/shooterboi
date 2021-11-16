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
        list,
        max_march_step_slider,
        ambient_occlusion_sample_slider,
        volume_slider,
        volume_tick_box,
        back_button
    }
}

pub struct SettingsScene {
    ids: SettingsSceneIds,
    ambient_occlusion_sample: u8,
    march_max_step: u8,
}

impl SettingsScene {
    pub fn new(renderer: &mut Renderer, conrod_handle: &mut ConrodHandle) -> Self {
        Self {
            ids: SettingsSceneIds::new(conrod_handle.get_ui_mut().widget_id_generator()),
            ambient_occlusion_sample: 2,
            march_max_step: 50,
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

        let mut ui_cell = conrod_handle.get_ui_mut().set_widgets();
        conrod_core::widget::Canvas::new()
            .pad(MARGIN)
            .set(self.ids.canvas, &mut ui_cell);

        for value in conrod_core::widget::Slider::new(self.march_max_step as f32, 0.0, 100.0)
            .label("Ambient occlusion sample")
            // .scroll_kids_horizontally()
            .mid_top_with_margin_on(self.ids.canvas, MARGIN)
            .wh(conrod_core::Dimensions::new(100.0, 100.0))
            .set(self.ids.max_march_step_slider, &mut ui_cell)
        {
            self.march_max_step = value as u8;
        }

        for value in
            conrod_core::widget::Slider::new(self.ambient_occlusion_sample as f32, 0.0, 10.0)
                .label("Ambient occlusion sample")
                // .scroll_kids_horizontally()
                .middle_of(self.ids.canvas)
                .wh(conrod_core::Dimensions::new(100.0, 100.0))
                .set(self.ids.ambient_occlusion_sample_slider, &mut ui_cell)
        {
            self.ambient_occlusion_sample = value as u8;
        }

        for _press in conrod_core::widget::Button::new()
            .label("Back")
            .bottom_left_with_margin_on(self.ids.canvas, MARGIN)
            .wh(conrod_core::Dimensions::new(100.0, 100.0))
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
