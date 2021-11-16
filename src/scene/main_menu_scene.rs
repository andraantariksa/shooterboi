use conrod_core::widget::envelope_editor::EnvelopePoint;
use conrod_core::{Labelable, Positionable, Sizeable, Widget};
use winit::event_loop::ControlFlow;

use crate::audio::{AudioContext, SINK_ID_MAIN_MENU_BGM};
use crate::gui::ConrodHandle;
use crate::input_manager::InputManager;
use crate::renderer::Renderer;
use crate::scene::classic_game_scene::ClassicGameScene;
use crate::scene::settings_scene::SettingsScene;
use crate::scene::{Scene, SceneOp, MARGIN};
use crate::window::Window;
use conrod_core::widget_ids;

widget_ids! {
    pub struct MainMenuSceneIds {
        // The main canvas
        canvas,
        quit_button,
        guide_button,
        settings_button,
        start_classic_mode_button
    }
}

pub struct MainMenuScene {
    ids: MainMenuSceneIds,
}

impl MainMenuScene {
    pub fn new(renderer: &mut Renderer, conrod_handle: &mut ConrodHandle) -> Self {
        Self {
            ids: MainMenuSceneIds::new(conrod_handle.get_ui_mut().widget_id_generator()),
        }
    }
}

impl Scene for MainMenuScene {
    fn init(
        &mut self,
        window: &mut Window,
        renderer: &mut Renderer,
        conrod_handle: &mut ConrodHandle,
        audio_context: &mut AudioContext,
    ) {
        renderer.is_render_gui = true;
        renderer.is_render_game = false;

        let mut sink_bgm = audio_context.get_sink_mut(SINK_ID_MAIN_MENU_BGM);
        if sink_bgm.empty() {
            sink_bgm.append(
                rodio::Decoder::new(std::io::Cursor::new(
                    include_bytes!("../../assets/audio/little-town.ogg").to_vec(),
                ))
                .unwrap(),
            );
        } else {
            sink_bgm.play();
        }
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

        let mut sink_bgm = audio_context.get_sink_mut(SINK_ID_MAIN_MENU_BGM);
        if sink_bgm.empty() {
            sink_bgm.append(
                rodio::Decoder::new(std::io::Cursor::new(
                    include_bytes!("../../assets/audio/little-town.ogg").to_vec(),
                ))
                .unwrap(),
            );
        } else {
            sink_bgm.play();
        }

        let settings_button;

        {
            let mut ui_cell = conrod_handle.get_ui_mut().set_widgets();

            conrod_core::widget::Canvas::new()
                .pad(MARGIN)
                .set(self.ids.canvas, &mut ui_cell);

            for _press in conrod_core::widget::Button::new()
                .label("Classic Mode")
                .wh(conrod_core::Dimensions::new(100.0, 100.0))
                .mid_left_with_margin_on(self.ids.canvas, MARGIN)
                .set(self.ids.start_classic_mode_button, &mut ui_cell)
            {
                scene_op = SceneOp::Push(Box::new(ClassicGameScene::new(renderer)));
            }

            settings_button = conrod_core::widget::Button::new()
                .label("Settings")
                .wh(conrod_core::Dimensions::new(100.0, 100.0))
                .bottom_left_with_margin_on(self.ids.canvas, MARGIN)
                .set(self.ids.settings_button, &mut ui_cell);

            for _press in conrod_core::widget::Button::new()
                .label("Guide")
                .wh(conrod_core::Dimensions::new(100.0, 100.0))
                .mid_bottom_with_margin_on(self.ids.canvas, MARGIN)
                .set(self.ids.guide_button, &mut ui_cell)
            {
                println!("Guide");
            }

            for _press in conrod_core::widget::Button::new()
                .label("Quit")
                .wh(conrod_core::Dimensions::new(100.0, 100.0))
                .bottom_right_with_margin_on(self.ids.canvas, MARGIN)
                .set(self.ids.quit_button, &mut ui_cell)
            {
                *control_flow = ControlFlow::Exit;
            }
        }

        for _press in settings_button {
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
