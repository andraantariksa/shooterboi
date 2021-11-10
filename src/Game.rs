enum SceneState {
    MainMenu,
    Settings,
    Game,
}

struct Game {
    scene_state: SceneState,
}

impl Game {
    fn new() -> Self {
        Self {
            scene_state: SceneState::MainMenu,
        }
    }
}
