use super::formatter::format_time;

use core::fmt::Debug;
use std::fmt::{Formatter, Result};
use std::time::{Duration, Instant};

pub struct TimerModule {
    is_running: bool,
    start_time: Instant,
    duration: Duration,
}

impl TimerModule {
    pub fn new() -> Self {
        TimerModule {
            is_running: false,
            start_time: Instant::now(),
            duration: Duration::new(0, 0),
        }
    }

    pub fn start(&mut self) -> &mut Self {
        self.update_start_time();
        self.is_running = true;
        self
    }

    pub fn pause(&mut self) -> &mut Self {
        self.update_duration();
        self.is_running = false;
        self
    }

    pub fn reset(&mut self) -> &mut Self {
        self.start_time = Instant::now();
        self.duration = Duration::new(0, 0);
        self.is_running = false;
        self
    }

    pub fn refresh(&mut self) -> &mut Self {
        self.reset();
        self.start();
        self
    }

    pub fn set_time(&mut self, time_seconds: u64) -> &mut Self {
        self.duration = Duration::new(time_seconds, 0);
        self.start_time = Instant::now() - self.duration;
        self
    }

    pub fn get_time(&mut self) -> f64 {
        self.update_duration();
        self.duration.as_secs_f64()
    }

    pub fn get_time_ms(&mut self) -> f64 {
        self.update_duration();
        self.duration.as_millis() as f64
    }

    pub fn get_string(&mut self) -> String {
        self.update_duration();
        let formatted_time: String = format_time(self.duration);
        formatted_time
    }

    fn update_start_time(&mut self) {
        if !self.is_running {
            self.start_time = Instant::now() - self.duration;
        }
    }

    fn update_duration(&mut self) {
        if self.is_running {
            self.duration = Instant::now() - self.start_time;
        }
    }
}

fn get_duration(time_module: &TimerModule) -> Duration {
    let mut duration: Duration = time_module.duration;
    if time_module.is_running {
        duration = Instant::now() - time_module.start_time;
    }
    duration
}

impl Debug for TimerModule {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let duration: Duration = get_duration(self);
        let formatted_time: String = format_time(duration);
        write!(f, "{}", formatted_time)
    }
}
