use crate::audio::{AudioContext, Sink, AUDIO_FILE_SHOOTED};
use crate::renderer::render_objects::MaterialType;
use crate::timer::Timer;
use nalgebra::{distance, Point3, Unit, Vector2, Vector3};
use std::io::{BufReader, Cursor};

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
pub enum Patrol {
    None,
    Linear {
        a: Vector3<f32>,
        b: Vector3<f32>,
    },
    Polar {
        or: Vector3<f32>,
        r: f32,
        a: f32,
        b: f32,
        c: f32,
    },
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
    patrol: Patrol,
    patrol_state: PatrolState,
}

pub const SPEED_LIN: f32 = 5.0;
pub const SPEED_POL: f32 = 0.3;

impl Target {
    pub fn new(validity: Option<Validity>, patrol: Patrol) -> Self {
        let validity_state = if let Some(x) = &validity {
            ValidityState::Valid(Timer::new(x.valid_duration))
        } else {
            ValidityState::None
        };
        let patrol_state = match patrol {
            Patrol::None => PatrolState::None,
            _ => PatrolState::AToB,
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
        patrol: Patrol,
    ) -> Self {
        let validity_state = if let Some(x) = &validity {
            ValidityState::Valid(Timer::new(x.valid_duration))
        } else {
            ValidityState::None
        };
        let patrol_state = match patrol {
            Patrol::None => PatrolState::None,
            _ => PatrolState::AToB,
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

    pub fn try_shoot(&mut self, audio_context: &mut AudioContext) -> bool {
        let sink = rodio::Sink::try_new(&audio_context.output_stream_handle).unwrap();
        sink.append(
            rodio::Decoder::new(BufReader::new(Cursor::new(AUDIO_FILE_SHOOTED.to_vec()))).unwrap(),
        );
        audio_context.push(Sink::Regular(sink));

        if !self.is_fake_target() {
            self.shooted = true;
            self.delete_timer = Some(Timer::new(0.3));
        }
        self.shooted
    }

    pub fn is_fake_target(&self) -> bool {
        match self.validity_state {
            ValidityState::Invalid(_) => true,
            _ => false,
        }
    }

    pub fn get_material(&self) -> MaterialType {
        match self.validity_state {
            ValidityState::Valid(_) | ValidityState::None => {
                if self.shooted {
                    MaterialType::TargetDimmed
                } else {
                    MaterialType::Target
                }
            }
            ValidityState::Invalid(_) => MaterialType::Yellow,
        }
    }

    pub fn update(&mut self, delta_time: f32, obj_pos: &mut Vector3<f32>) {
        match &mut self.validity_state {
            ValidityState::Valid(x) => {
                x.update(delta_time);
                if x.is_finished() {
                    self.validity_state = ValidityState::Invalid(Timer::new(
                        self.validity.as_ref().unwrap().invalid_duration,
                    ));
                }
            }
            ValidityState::Invalid(x) => {
                x.update(delta_time);
                if x.is_finished() {
                    self.validity_state = ValidityState::Valid(Timer::new(
                        self.validity.as_ref().unwrap().valid_duration,
                    ));
                }
            }
            ValidityState::None => {}
        }

        match &mut self.patrol {
            Patrol::Linear { a, b } => {
                match &self.patrol_state {
                    PatrolState::AToB => {
                        if distance(&Point3::from(obj_pos.clone()), &Point3::from(b.clone())) <= 0.5
                        {
                            self.patrol_state = PatrolState::BToA;
                        } else {
                            let dir = Unit::new_normalize(*b - obj_pos.clone());
                            let next_pos = dir.into_inner() * SPEED_LIN * delta_time;
                            obj_pos.x += next_pos.x;
                            obj_pos.z += next_pos.y;
                        }
                    }
                    PatrolState::BToA => {
                        if distance(&Point3::from(obj_pos.clone()), &Point3::from(a.clone())) <= 0.5
                        {
                            self.patrol_state = PatrolState::AToB;
                        } else {
                            let dir = Unit::new_normalize(*a - obj_pos.clone());
                            let next_pos = dir.into_inner() * SPEED_LIN * delta_time;
                            obj_pos.x += next_pos.x;
                            obj_pos.z += next_pos.y;
                        }
                    }
                    PatrolState::None => unreachable!(),
                };
            }
            Patrol::Polar {
                ref a,
                ref b,
                c,
                ref or,
                ref r,
            } => {
                match &self.patrol_state {
                    PatrolState::AToB => {
                        if (*c - b).abs() <= SPEED_POL {
                            //println!("State B to A {} {} {}", a, b, c);
                            self.patrol_state = PatrolState::BToA;
                        } else {
                            let mut sign = (b - *c).signum();
                            *c = *c + sign * SPEED_POL * delta_time;
                            obj_pos.x = r * c.cos();
                            obj_pos.z = r * c.sin();
                            obj_pos.x += or.x;
                            obj_pos.z += or.z;
                        }
                    }
                    PatrolState::BToA => {
                        if (*c - a).abs() <= SPEED_POL {
                            //println!("State A to B {} {} {}", a, b, c);
                            self.patrol_state = PatrolState::AToB;
                        } else {
                            let mut sign = (a - *c).signum();
                            *c = *c + sign * SPEED_POL * delta_time;
                            obj_pos.x = r * c.cos();
                            obj_pos.z = r * c.sin();
                            obj_pos.x += or.x;
                            obj_pos.z += or.z;
                        }
                    }
                    PatrolState::None => unreachable!(),
                };
            }
            _ => {}
        };

        match self.delete_timer {
            Some(ref mut timer) => {
                timer.update(delta_time);
            }
            None => {}
        };
    }

    pub fn is_need_to_be_deleted(&self) -> bool {
        match self.delete_timer {
            Some(ref timer) => timer.is_finished(),
            None => false,
        }
    }
}
