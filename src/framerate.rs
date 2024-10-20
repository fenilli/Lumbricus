pub struct Framerate {
    last_frame_time: std::time::Instant,
    delta_time: std::time::Duration,
}

impl Framerate {
    pub fn new() -> Self {
        Self {
            last_frame_time: std::time::Instant::now(),
            delta_time: std::time::Duration::ZERO,
        }
    }

    pub fn tick(&mut self) -> f64 {
        let now = std::time::Instant::now();
        self.delta_time = now - self.last_frame_time;
        self.last_frame_time = now;

        self.delta_time.as_secs_f64()
    }

    pub fn get_frames_per_second(&self) -> u64 {
        let delta_ns = self.delta_time.as_nanos();
        if delta_ns > 0 {
            (1_000_000_000 / delta_ns) as u64
        } else {
            u64::MAX
        }
    }
}
