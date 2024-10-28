use std::time::{Duration, Instant};

pub struct FrameTimer {
    last_frame: Instant,
    time_accumulator: Duration,
    fixed_time_step: Duration,
    delta_time: Duration,
}

impl FrameTimer {
    pub fn new(fps: u32) -> Self {
        let fixed_time_step = Duration::from_secs_f32(1.0 / if fps > 0 { fps } else { 60 } as f32);

        Self {
            last_frame: Instant::now(),
            time_accumulator: Duration::ZERO,
            fixed_time_step,
            delta_time: Duration::ZERO,
        }
    }

    pub fn tick(&mut self) {
        let now = Instant::now();
        self.delta_time = now - self.last_frame;
        self.last_frame = now;
        self.time_accumulator += self.delta_time;
    }

    pub fn should_do_fixed_tick(&mut self) -> bool {
        if self.time_accumulator >= self.fixed_time_step {
            self.time_accumulator -= self.fixed_time_step;

            true
        } else {
            false
        }
    }

    pub fn get_delta_time(&self) -> f32 {
        self.delta_time.as_secs_f32()
    }

    pub fn get_fixed_delta_time(&self) -> f32 {
        self.fixed_time_step.as_secs_f32()
    }
}
