use crate::renderer::render_objects::MaterialType;
use crate::timer::Timer;

#[derive(Clone)]
pub struct Target {
    shooted: bool,
    delete_timer: Option<Timer>,
    fake: bool,
}

impl Target {
    pub fn new(fake: bool) -> Self {
        Self {
            shooted: false,
            delete_timer: None,
            fake,
        }
    }

    pub fn new_with_delete_duration(timer: Timer) -> Self {
        Self {
            shooted: false,
            delete_timer: Some(timer),
            fake: false,
        }
    }

    pub fn is_shooted(&self) -> bool {
        self.shooted
    }

    pub fn shooted(&mut self) {
        self.shooted = true;
        self.delete_timer = Some(Timer::new(0.3));
    }

    pub fn get_material(&self) -> MaterialType {
        if self.shooted {
            MaterialType::TargetDimmed
        } else {
            MaterialType::Target
        }
    }

    pub fn is_need_to_be_deleted(&mut self, delta_time: f32) -> bool {
        match self.delete_timer {
            Some(ref mut timer) => {
                timer.update(delta_time);
                timer.is_finished()
            }
            None => false,
        }
    }
}
