use crate::timer::Timer;

#[derive(Clone)]
pub enum InOutAnimationState {
    Stopped,
    Foward(Timer),
    Backward(Timer),
}

impl PartialEq for InOutAnimationState {
    fn eq(&self, other: &Self) -> bool {
        match self {
            InOutAnimationState::Stopped => matches!(other, InOutAnimationState::Stopped),
            InOutAnimationState::Foward(_) => matches!(other, InOutAnimationState::Foward(_)),
            InOutAnimationState::Backward(_) => matches!(other, InOutAnimationState::Backward(_)),
        }
    }
}

pub struct InOutAnimation {
    sec_to_forward: f32,
    sec_to_backward: f32,
    state: InOutAnimationState,
}

impl InOutAnimation {
    pub fn new(sec_to_forward: f32, sec_to_backward: f32) -> Self {
        Self {
            sec_to_forward,
            sec_to_backward,
            state: InOutAnimationState::Stopped,
        }
    }

    pub fn new_started(sec_to_forward: f32, sec_to_backward: f32) -> Self {
        Self {
            sec_to_forward,
            sec_to_backward,
            state: InOutAnimationState::Foward(Timer::new(sec_to_forward)),
        }
    }

    pub fn trigger(&mut self) {
        self.state = InOutAnimationState::Foward(Timer::new(self.sec_to_forward));
    }

    pub fn update(&mut self, delta_time: f32) {
        match &mut self.state {
            InOutAnimationState::Foward(timer) => {
                let duration_left = timer.get_duration();
                if delta_time > duration_left {
                    self.state = InOutAnimationState::Backward(Timer::new(
                        self.sec_to_backward + delta_time - duration_left,
                    ));
                    return;
                }
                timer.update(delta_time);
            }
            InOutAnimationState::Backward(timer) => {
                timer.update(delta_time);
                if timer.is_finished() {
                    self.state = InOutAnimationState::Stopped;
                }
            }
            _ => {}
        };
    }

    pub fn get_value(&self) -> f32 {
        match &self.state {
            InOutAnimationState::Stopped => 0.0,
            InOutAnimationState::Foward(timer) => 1.0 - timer.get_duration() / self.sec_to_forward,
            InOutAnimationState::Backward(timer) => timer.get_duration() / self.sec_to_backward,
        }
    }

    pub fn get_state(&self) -> InOutAnimationState {
        self.state.clone()
    }
}
