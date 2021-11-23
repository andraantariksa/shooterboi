use instant::{Duration, Instant};

pub struct Timer {
    duration: Duration,
    last_update: Option<Instant>,
}

impl Timer {
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            last_update: None,
        }
    }

    pub fn new_finished() -> Self {
        Self {
            duration: Duration::new(0, 0),
            last_update: None,
        }
    }

    pub fn new_start(duration: Duration) -> Self {
        let mut self_ = Self {
            duration,
            last_update: None,
        };
        self_.start();
        self_
    }

    pub fn reset(&mut self, duration: Duration) {
        self.duration = duration;
        self.start();
    }

    pub fn pause(&mut self) {
        self.last_update = None;
    }

    pub fn start(&mut self) {
        self.last_update = Some(Instant::now());
    }

    pub fn update(&mut self) {
        if let Some(last_update) = self.last_update {
            self.duration = self
                .duration
                .saturating_sub(Instant::now().duration_since(last_update));
            self.last_update = Some(Instant::now());
        }
    }

    pub fn is_finished(&self) -> bool {
        self.duration.is_zero()
    }

    pub fn get_duration(&self) -> &Duration {
        &self.duration
    }
}
