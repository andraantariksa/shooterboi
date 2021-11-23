use conrod_core::widget::envelope_editor::EnvelopePoint;
use conrod_core::{Labelable, Positionable, Sizeable, Widget};
use winit::event_loop::ControlFlow;

use crate::audio::AudioContext;
use crate::gui::ConrodHandle;
use crate::input_manager::InputManager;
use crate::renderer::Renderer;
use crate::scene::settings_scene::SettingsScene;
use crate::scene::{Scene, SceneOp, MARGIN};
use crate::window::Window;
use conrod_core::widget_ids;

widget_ids! {
    pub struct PauseSceneIds {
        // The main canvas
        canvas,
        resume_button,
        settings_button,
        quit_buton
    }
}

pub struct PauseScene {
    ids: PauseSceneIds,
}

impl PauseScene {
    pub fn new(renderer: &mut Renderer, conrod_handle: &mut ConrodHandle) -> Self {
        Self {
            ids: PauseSceneIds::new(conrod_handle.get_ui_mut().widget_id_generator()),
        }
    }
}

impl Scene for PauseScene {
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

        let settings_button;

        {
            let ropa_font_id = *conrod_handle.get_font_id_map().get("ropa").unwrap();
            let mut ui_cell = conrod_handle.get_ui_mut().set_widgets();
            conrod_core::widget::Canvas::new()
                .pad(MARGIN)
                .set(self.ids.canvas, &mut ui_cell);

            settings_button = conrod_core::widget::Button::new()
                .label("Settings")
                .label_font_id(ropa_font_id)
                .middle_of(self.ids.canvas)
                .wh(conrod_core::Dimensions::new(100.0, 30.0))
                .set(self.ids.settings_button, &mut ui_cell);

            for _ in conrod_core::widget::Button::new()
                .label("Resume")
                .label_font_id(ropa_font_id)
                .up_from(self.ids.settings_button, 30.0)
                .wh(conrod_core::Dimensions::new(100.0, 30.0))
                .set(self.ids.resume_button, &mut ui_cell)
            {
                scene_op = SceneOp::Pop(1);
            }

            for _ in conrod_core::widget::Button::new()
                .label("Quit")
                .label_font_id(ropa_font_id)
                .down_from(self.ids.settings_button, 30.0)
                .wh(conrod_core::Dimensions::new(100.0, 30.0))
                .set(self.ids.quit_buton, &mut ui_cell)
            {
                scene_op = SceneOp::Pop(2);
            }
        }

        for _ in settings_button {
            scene_op = SceneOp::Push(Box::new(SettingsScene::new(renderer, conrod_handle)));
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
