use crate::logging;
use crate::api::{Part, Status, Ticker};

/// Represents a show's properties.
pub struct Show<'a> {
    /// The name of the show.
    name: &'a str,
    /// The parts included in the show.
    parts: Vec<&'a mut dyn Part>,
    /// The show's tick speed.
    tick_speed: u32,
    /// The current part index.
    current_part: usize,
}

impl Show<'_> {
    /// Starts the show. This is a blocking function, and will only stop
    /// when the show is terminated by the user.
    pub fn start(&mut self) {
        let mut p: &mut dyn Part;
        let mut result: Status;
        let mut ticker = Ticker::new();
        loop {
            ticker.tick(self.tick_speed);
            p = self.parts[self.current_part];
            result = p.on_update();
            match result {
                Status::STOP => {
                    logging::info("Show is stopped by show logic.");
                    std::process::exit(0);
                }
                Status::SKIP => {
                    if self.current_part >= self.parts.len() {
                        logging::info("There are no more parts in the show. Stopping.");
                        std::process::exit(0);

                    } else {
                        self.current_part += 1;
                    }
                }
                Status::REWIND => {
                    if self.current_part <= 0 {
                        logging::info("There are no more parts before the current part. Stopping.");
                        std::process::exit(0);
                    } else {
                        self.current_part -= 1;
                    }
                }
                _ => {}
            }
        }
    }

    /// Creates a new show.
    pub fn new<'a>(name: &'a str, parts: Vec<&'a mut dyn Part>, tick_speed: u32) -> Show<'a> {
        Show {
            name,
            parts,
            tick_speed,
            current_part: 0,
        }
    }
}
