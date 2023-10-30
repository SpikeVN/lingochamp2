use std::time::{Duration, SystemTime};

pub struct Timer {
    start_time: Option<SystemTime>,
    paused_time: Option<SystemTime>,
    paused_duration: Duration,
    is_paused: bool,
}

impl Timer {
    /// Sets the timer starting point.
    pub fn new() -> Timer {
        Timer {
            start_time: Some(SystemTime::now()),
            paused_duration: Duration::new(0, 0),
            paused_time: None,
            is_paused: false,
        }
    }

    /// Pauses the timer.
    pub fn pause(&mut self) {
        self.start_time.expect("Timer object is uninitialized.");
        self.paused_time = Some(SystemTime::now());
    }

    /// Returns the time elapsed from the calling of `start()`.
    pub fn time_elapsed(&self) -> Duration {
        let base_dur: Option<Duration>;
        if self.is_paused {
            base_dur = Some(
                self.start_time
                    .expect("Timer object is uninitialized.")
                    .duration_since(self.paused_time.expect("Time is not paused. Impossible."))
                    .expect("Error when calculating time elapsed of Timer object."),
            );
        } else {
            base_dur = Some(
                SystemTime::now()
                    .duration_since(self.start_time.expect("Timer object is uninitialized."))
                    .expect("Error when calculating time elapsed of Timer object."),
            )
        }
        base_dur.unwrap() - self.paused_duration
    }
}
