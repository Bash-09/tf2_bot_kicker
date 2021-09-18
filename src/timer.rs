#![allow(dead_code)]

use std::time::Instant;


pub struct Timer {
    last: Instant,
    interval_duration: f32,
    current: f32,
    intervals: u32,
    pub done: bool,
}

impl Timer {

    pub fn new() -> Timer {
        Timer {
            last: Instant::now(),
            interval_duration: 1.0,
            current: 0.0,
            intervals: 0,
            done: false,
        }

    }

    pub fn go(&mut self) -> bool {
        let now = self.last.elapsed();
        let delta = (now.as_micros() as f32) / 1_000_000.0;

        self.current += delta;

        self.last = Instant::now();

        if self.current > self.interval_duration {
            self.current = 0.0;
            self.intervals += 1;
            self.done = false;
            return true;
        }

        return false;
    }

    pub fn reset(&mut self) {
        self.last = Instant::now();
        self.current = 0.0;
    }

    pub fn set_interval_duration(&mut self, interval: f32) {
        self.interval_duration = interval;
    }

    pub fn intervals(&self) -> u32 {
        self.intervals
    }

}