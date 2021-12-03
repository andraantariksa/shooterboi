use instant::{Duration, Instant};

pub struct Timer {
    duration: f32,
}

impl Timer {
    pub fn new(duration: f32) -> Self {
        Self { duration }
    }

    pub fn new_finished() -> Self {
        Self { duration: 0.0 }
    }

    pub fn reset(&mut self, duration: f32) {
        self.duration = duration;
    }

    pub fn update(&mut self, duration: f32) {
        self.duration = (self.duration - duration).max(0.0);
    }

    pub fn is_finished(&self) -> bool {
        self.duration <= 0.0
    }

    pub fn get_duration(&self) -> f32 {
        self.duration
    }
}

pub struct Stopwatch {
    duration: f32,
}

impl Stopwatch {
    pub fn new() -> Self {
        Self { duration: 0.0 }
    }

    pub fn reset(&mut self) {
        self.duration = 0.0;
    }

    pub fn update(&mut self, duration: f32) {
        self.duration += duration;
    }

    pub fn get_duration(&self) -> f32 {
        self.duration
    }
}
