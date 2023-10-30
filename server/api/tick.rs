use std::{thread, time::{Duration, SystemTime}};

pub struct Ticker {
    last_tick: SystemTime
}

impl Ticker {
    pub fn new() -> Ticker {
        Ticker {last_tick: SystemTime::now()}
    }
    /// Ensures the tick speed is met.
    pub fn tick(&mut self, tick_speed: u32) {
        let code_runtime = SystemTime::now().duration_since(self.last_tick).expect("Error when calculating time elapsed for regulating tick speed");
        let sleep_dur = Duration::from_millis(((1 as f32 / tick_speed as f32) * 1000 as f32) as u64);
        thread::sleep(sleep_dur - code_runtime);
        self.last_tick = SystemTime::now();
    }
}