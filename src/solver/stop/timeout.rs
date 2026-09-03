use std::time::{Duration, Instant};

use super::StopCondition;

pub struct TimeoutCondition {
    duration: Duration,
    start_time: Instant,
}

impl TimeoutCondition {
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            start_time: Instant::now(),
        }
    }
}

impl StopCondition for TimeoutCondition {
    fn should_stop(&self) -> bool {
        self.start_time.elapsed() >= self.duration
    }

    fn reset(&mut self) {
        self.start_time = Instant::now();
    }
}
