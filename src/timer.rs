use std::time::{Duration, Instant};

#[allow(dead_code)]
pub struct Timer {
    current_frame_time: Duration,
    last_frame_time: Instant,
}

#[allow(dead_code)]
impl Timer {
    pub fn new() -> Self {
        Self {
            current_frame_time: Duration::ZERO,
            last_frame_time: Instant::now(),
        }
    }

    pub fn tick(&mut self) -> f64 {
        let now = Instant::now();
        self.current_frame_time = now - self.last_frame_time;
        self.last_frame_time = now;

        self.current_frame_time.as_secs_f64()
    }

    pub fn get_fps(&self) -> u64 {
        if self.current_frame_time.as_secs_f64() > 0.0 {
            (1.0 / self.current_frame_time.as_secs_f64()) as u64
        } else {
            u64::MAX
        }
    }
}
