use crate::renderer::render_objects::MaterialType;
use crate::timer::Timer;
use nalgebra::Vector3;

#[derive(Clone)]
pub struct Validity {
    pub valid_duration: f32,
    pub invalid_duration: f32,
}

#[derive(Clone)]
pub enum ValidityState {
    Valid(Timer),
    Invalid(Timer),
    None,
}

#[derive(Clone)]
pub struct Patrol {
    pub a: Vector3<f32>,
    pub b: Vector3<f32>,
}

#[derive(Clone)]
enum PatrolState {
    AToB,
    BToA,
    None,
}

#[derive(Clone)]
pub struct Target {
    shooted: bool,
    delete_timer: Option<Timer>,
    validity: Option<Validity>,
    validity_state: ValidityState,
    patrol: Option<Patrol>,
    patrol_state: PatrolState,
}

impl Target {
    pub fn new(validity: Option<Validity>, patrol: Option<Patrol>) -> Self {
        let validity_state = if let Some(x) = &validity {
            ValidityState::Valid(Timer::new(x.valid_duration))
        } else {
            ValidityState::None
        };
        let patrol_state = if patrol.is_some() {
            PatrolState::AToB
        } else {
            PatrolState::None
        };
        Self {
            shooted: false,
            delete_timer: None,
            validity,
            validity_state,
            patrol,
            patrol_state,
        }
    }

    pub fn new_with_delete_duration(
        timer: Timer,
        validity: Option<Validity>,
        patrol: Option<Patrol>,
    ) -> Self {
        let validity_state = if let Some(x) = &validity {
            ValidityState::Valid(Timer::new(x.valid_duration))
        } else {
            ValidityState::None
        };
        let patrol_state = if patrol.is_some() {
            PatrolState::AToB
        } else {
            PatrolState::None
        };
        Self {
            shooted: false,
            delete_timer: Some(timer),
            validity,
            validity_state,
            patrol,
            patrol_state,
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

    pub fn update(&mut self, delta_time: f32) {
        match &mut self.validity_state {
            ValidityState::Valid(x) => {
                x.update(delta_time);
            }
            ValidityState::Invalid(x) => {
                x.update(delta_time);
            }
            ValidityState::None => {}
        }

        match &mut self.patrol_state {
            PatrolState::AToB => {}
            PatrolState::BToA => {}
            PatrolState::None => {}
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
