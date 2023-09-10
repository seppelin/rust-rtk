use std::time::{Duration, Instant};

pub struct Timer {
    start: Instant,
    interval: Duration,
}

impl Timer {
    pub fn new(interval: Duration) -> Self {
        Self {
            start: Instant::now(),
            interval,
        }
    }

    pub fn is_done(&mut self) -> bool {
        if Instant::now() - self.start >= self.interval {
            self.start = Instant::now();
            true
        } else {
            false
        }
    }
}
