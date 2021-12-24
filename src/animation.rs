#[derive(Clone)]
pub enum InOutAnimationState {
    Stopped,
    Foward,
    Backward,
}

pub struct InOutAnimation {
    value: f32,
    sec_to_forward: f32,
    sec_to_backward: f32,
    state: InOutAnimationState,
}

impl InOutAnimation {
    pub fn new(sec_to_forward: f32, sec_to_backward: f32) -> Self {
        Self {
            value: 0.0,
            sec_to_forward,
            sec_to_backward,
            state: InOutAnimationState::Stopped,
        }
    }

    pub fn new_started(sec_to_forward: f32, sec_to_backward: f32) -> Self {
        Self {
            value: 0.0,
            sec_to_forward,
            sec_to_backward,
            state: InOutAnimationState::Foward,
        }
    }

    pub fn trigger(&mut self) {
        self.state = InOutAnimationState::Foward;
    }

    pub fn update(&mut self, delta_time: f32) {
        match self.state {
            InOutAnimationState::Foward => {
                self.value += delta_time * (100.0 / self.sec_to_forward);
                if self.value >= 1.0 {
                    self.value -= self.value % 1.0;
                    self.state = InOutAnimationState::Backward;
                }
            }
            InOutAnimationState::Backward => {
                self.value -= delta_time * (100.0 / self.sec_to_backward);
                if self.value < 0.0 {
                    self.value = 0.0;
                    self.state = InOutAnimationState::Stopped;
                }
            }
            _ => {}
        };
    }

    pub fn get_value(&self) -> f32 {
        self.value
    }

    pub fn get_state(&self) -> InOutAnimationState {
        self.state.clone()
    }
}
