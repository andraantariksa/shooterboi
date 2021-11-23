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
use winit::event::VirtualKeyCode;

widget_ids! {
    pub struct GuideSceneIds {
        // The main canvas
        canvas,
        buttons_canvas,
        confirm_label,
        yes_button,
        no_button
    }
}

pub struct GuideScene {
    ids: GuideSceneIds,
}

impl GuideScene {
    pub fn new(renderer: &mut Renderer, conrod_handle: &mut ConrodHandle) -> Self {
        Self {
            ids: GuideSceneIds::new(conrod_handle.get_ui_mut().widget_id_generator()),
        }
    }
}

impl Scene for GuideScene {
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

        let no_button;
        let yes_button;

        {
            let mut ui_cell = conrod_handle.get_ui_mut().set_widgets();
            conrod_core::widget::Canvas::new().set(self.ids.canvas, &mut ui_cell);

            conrod_core::widget::Canvas::new()
                .middle_of(self.ids.canvas)
                .set(self.ids.buttons_canvas, &mut ui_cell);

            yes_button = conrod_core::widget::Button::new()
                .label("Yes")
                .align_right_of(self.ids.buttons_canvas)
                .wh(conrod_core::Dimensions::new(100.0, 30.0))
                .set(self.ids.yes_button, &mut ui_cell);

            no_button = conrod_core::widget::Button::new()
                .label("No")
                .align_left_of(self.ids.buttons_canvas)
                .wh(conrod_core::Dimensions::new(100.0, 30.0))
                .set(self.ids.no_button, &mut ui_cell);
        }

        if input_manager.is_keyboard_press(&VirtualKeyCode::Escape) {
            scene_op = SceneOp::Pop(1);
        }

        for _press in yes_button {
            *control_flow = ControlFlow::Exit;
        }

        for _press in no_button {
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
