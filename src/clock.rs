use std::time::{Duration, Instant};

pub struct Clock {
    last_frame: Instant,
    delta_time: Duration,
}

impl Clock {
    pub fn new() -> Self {
        Self {
            last_frame: Instant::now(),
            delta_time: Duration::ZERO,
        }
    }

    pub fn tick(&mut self) {
        let now = Instant::now();
        self.delta_time = now - self.last_frame;
        self.last_frame = now;
    }

    pub fn get_delta_time(&self) -> f32 {
        self.delta_time.as_secs_f32()
    }
}
